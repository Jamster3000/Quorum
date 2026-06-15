import zstandard as zstd
import struct

#Magic number that's always identical in all headers for each compressed message
ZSTD_MAGIC = b'\x28\xb5\x2f\xfd'

#The descriptor in header describes the shape of the zstd frame, understands what constants shoudl be/are present and how big it is
def get_descriptor_info(full: bytes):
    descriptor = full[4]
    dict_id_flag = descriptor & 0x03
    single_segment_flag = (descriptor >> 5) & 0x01
    fcs_flag = (descriptor >> 6) & 0x03

    dict_id_sizes = {0: 0, 1: 1, 2: 2, 3: 4}
    fcs_sizes = {0: (1 if single_segment_flag else 0), 1: 2, 2: 4, 3: 8}

    dict_id_size = dict_id_sizes[dict_id_flag]
    fcs_size = fcs_sizes[fcs_flag]
    window_size = 0 if single_segment_flag else 1
    header_size = 4 + 1 + window_size + dict_id_size + fcs_size

    return descriptor, dict_id_size, fcs_flag, fcs_size, window_size, header_size

# Get all constants from the header
def extract_probe_constants(full: bytes):
    descriptor, dict_id_size, fcs_flag, fcs_size, window_size, header_size = get_descriptor_info(full)

    dict_id_offset = 5 + window_size
    dict_id = full[dict_id_offset:dict_id_offset + dict_id_size]

    return {
        "magic": full[0:4],
        "descriptor": descriptor,
        "dict_id": dict_id,
        "dict_id_size": dict_id_size,
        "window_size": window_size,
        "fcs_flag": fcs_flag,
        "fcs_size": fcs_size,
        "header_size": header_size,
    }

#this is the actual compression
def compress_minimal(text: str, dict_data) -> bytes:
    cctx = zstd.ZstdCompressor(level=1, dict_data=dict_data)
    full = cctx.compress(text.encode())

    descriptor, dict_id_size, fcs_flag, fcs_size, window_size, header_size = get_descriptor_info(full)
    original_len = len(text.encode())

    block_hdr = full[header_size:header_size + 3]
    payload = full[header_size + 3:]

    stored = struct.pack('!BBH', descriptor, fcs_size, original_len) + block_hdr + payload
    return stored

#Build the Frame Content Size given original length and how many bytes
def build_fcs_bytes(original_len: int, fcs_size: int) -> bytes:
    if fcs_size == 0:
        return b''
    if fcs_size == 1:
        return struct.pack('<B', original_len)
    if fcs_size == 2:
        if original_len < 256:
            raise ValueError("2-byte FCS requires original_len >= 256")
        return struct.pack('<H', original_len - 256)
    if fcs_size == 4:
        return struct.pack('<I', original_len)
    if fcs_size == 8:
        return struct.pack('<Q', original_len)
    raise ValueError(f"Unsupported fcs_size: {fcs_size}")

#This is the decompression
def decompress_minimal(stored: bytes, dict_data, magic: bytes, dict_id: bytes, window_size: int):
    descriptor = stored[0]
    fcs_size = stored[1]
    original_len = struct.unpack('!H', stored[2:4])[0]
    block_hdr = stored[4:7]
    payload = stored[7:]

    content_size = build_fcs_bytes(original_len, fcs_size)

    if window_size == 0:
        window_desc = b''
    else:
        raise ValueError("This compact reconstructor currently only supports window_size == 0")

    full = magic + bytes([descriptor]) + window_desc + dict_id + content_size + block_hdr + payload

    dctx = zstd.ZstdDecompressor(dict_data=dict_data)
    return dctx.decompress(full).decode()

# Test messages, these can be replaced with absolutely any length and any string
test_messages = [
    "Hey, are you free tonight?",
    "Did you see what happened in the last session?",
    "I'll be online in like 10 minutes",
    "the dragon rolled a nat 20 lmao we're all dead",
    "Hey, are you free tonight? Did you see what happend in the last session? I'll be online in like 10 minutes the dragon rolled a nat 20 lmao we're all dead",
    "This is a long string that should compress quite well. " * 5,
]

#samples and dict_data take the test_messages builks it out so the training
#has meaningful quantity and creates the dictionray for compression
samples = [msg.encode() for msg in test_messages * 20]
dict_data = zstd.train_dictionary(4096, samples)

cctx_probe = zstd.ZstdCompressor(level=19, dict_data=dict_data) #initilise compressor
probe = cctx_probe.compress(test_messages[0].encode())
probe_info = extract_probe_constants(probe)

MAGIC = probe_info["magic"] #the magic number in the header which doesn't need to be directly part of the compressed bytes.
DICT_ID = probe_info["dict_id"] #train_dictionary also returns dict_id this changes whenever the dict is retrained
WINDOW_SIZE = probe_info["window_size"]

print(f"MAGIC:   {MAGIC.hex()}")
print(f"DICT_ID: {DICT_ID.hex()}")

print(f"\n{'Message':<52} {'Orig':>6} {'Before':>8} {'After':>7} {'Saved':>7}")
print("-" * 86)

results = []
for msg in test_messages:
    cctx = zstd.ZstdCompressor(level=1, dict_data=dict_data)
    before = cctx.compress(msg.encode())
    after = compress_minimal(msg, dict_data)
    preview = msg[:50] + ".." if len(msg) > 50 else msg
    saved = len(before) - len(after)
    print(f"{preview:<52} {len(msg.encode()):>6} {len(before):>8} {len(after):>7} {saved:>7}")
    results.append((msg, after))

print("\n--- Round trip verification ---")
all_ok = True
for original_msg, stored in results:
    recovered = decompress_minimal(stored, dict_data, MAGIC, DICT_ID, WINDOW_SIZE)
    status = 'OK' if recovered == original_msg else 'FAIL'
    if status == 'FAIL':
        all_ok = False
    print(f"{status}  |  {original_msg[:60]}")

print(f"\nAll OK: {all_ok}")


'''
## Database storage workings
In a database, when a message is stored (e.g., a user sending a message)
the `after` variable is saved as bytes (not string) in a database.
There should be two tables ideally.

- Table One (conversation)
This table used to store the message/string, this can include any required rows, for the actual message content, this would be a byte data type (NOT STRING)

- Table Two (compression constants)
This table would hole the constants for compression (this table is also known as
normilization) as we don't need to store compression constrants in the database repeatedly if they're always the same.

This table will hold
    - `magic number` - `bytes` - this shouldn't change, so it could be hardcoded but it's good to have it in stored in database instead.
    - ` dict_id` - `bytes` - `train_dictionary` generates this when it's given a dictionary to train on.
    - `dict_blob` - `bytes` - this is a basic directory to train zstd on.
                            (there can be more than one dictionary for different
                            types of things that will be compressed) (e.g., try compressing
                            code as a string, it isn't very effective because the current dictionary
                            in this code isn't optimized for that).

'''
