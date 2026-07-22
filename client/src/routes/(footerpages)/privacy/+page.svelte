<script lang="ts">
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import { IconShield, IconMapPin, IconUsers, IconLock, IconAlertCircle, IconFileText, IconMail } from '@tabler/icons-svelte-runes';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
  <Card padding="xlarge" width="50%" variant="gradient">
      <article>
        <h1>Privacy Policy</h1>

        <section>
          <h2>Scope</h2>
          <p>
            This privacy policy applies to the Quorum application for Windows, together with any
            related services operated by Jamster (collectively, the "Application"). Jamster
            is the "Service Provider".
          </p>
        </section>

        <section>
          <h2>Information Collection and Use</h2>
          <p>
            The Application does not collect, log, or store any personal information when you
            download and use it. Registration is not required. If the Application is used with
            an active internet connection, technical protocol data (such as your ephemeral IP
            address) is transmitted to facilitate network connectivity, but this data is not
            retained or used for tracking.
          </p>
        </section>

        <section>
          <h2>Location Information</h2>
          <p>
            This Application does not collect precise information about the location of your device.
          </p>
        </section>

        <section>
          <h2>Third-Party Access</h2>
          <p>
            Since the Application does not collect any information, no data is shared with third parties.
          </p>
        </section>

        <section>
          <h2>Opt-Out Rights</h2>
          <p>
            Since the Application does not collect personal information through normal use,
            uninstalling it simply removes the Application from your device.
          </p>
          <p>
            If you contact the Service Provider directly or voluntarily provide information by
            other means, you may request deletion of that information by
            contacting <a href="mailto:[Email Address Placeholder]">[Email Address Placeholder]</a>.
          </p>
        </section>

        <section>
          <h2>Children</h2>
          <p>
            The Application is not intended for children under 16 years of age, or such
            higher age as required by applicable law. The Service Provider does not
            knowingly solicit data from children or market to them. Since the Application
            does not collect personal information through normal use, children's data is
            not at risk from use of the Application alone. If you voluntarily provide
            personal information through other means and are under 16 years of age,
            your parent or guardian must provide consent on your behalf where permitted by law.
          </p>
        </section>

        <section>
          <h2>Security</h2>
          <p>
            Because the Application does not collect personal data, the risk of personal
            data exposure is minimal. However, no security system is completely secure.
            The Service Provider implements reasonable safeguards to protect systems and
            any data it holds.
          </p>
        </section>

        <section>
          <h2>Data Breach Notification</h2>
          <p>
            Since the Application does not collect personal data through normal use, the risk of a data breach affecting your personal data is minimal. If a breach occurs involving any data you have voluntarily provided, the Service Provider will notify you as required by applicable law.
          </p>
        </section>

        <section>
          <h2>Policy Changes</h2>
          <p>
            The Service Provider may update this Privacy Policy from time to time. The Service Provider will notify you of material changes by posting the updated Privacy Policy with an effective date. Where required by law, the Service Provider will seek your consent to material changes before they take effect.
          </p>
          <p>
            Previous versions of this Privacy Policy will be maintained and made available upon request by contacting the Service Provider at <a href="mailto:[Email Address Placeholder]">[Email Address Placeholder]</a>.
          </p>
        </section>

        <section>
          <h2>Your Consent</h2>
          <p>
            If you voluntarily provide information to the Service Provider and processing is based on consent, you may withdraw that consent at any time without affecting processing carried out before withdrawal.
          </p>
        </section>

        <section>
          <h2>Contact Us</h2>
          <p>
            If you have any questions regarding privacy while using the Application, or have questions about the practices, please contact the Service Provider via email at <a href="mailto:[Email Address Placeholder]">[Email Address Placeholder]</a>.
          </p>
        </section>

        <footer>
          <p><small>Effective as of 2026-07-22</small></p>
          <p><small>Generated with <a href="https://app-privacy-policy-generator.nisrulz.com/" target="_blank" rel="noopener noreferrer">App Privacy Policy Generator</a></small></p>
        </footer>
      </article>
  </Card>
</main>

<Footer />

<style>
  section {
    margin-bottom: 4rem;
  }

  section:last-of-type {
    margin-bottom: 0;
  }

  h1 {
    margin-bottom: 2.5rem;
    font-size: var(--font-xlarge);
  }

  h2 {
    margin-bottom: 1.5rem;
    font-size: var(--font-large);
  }

  p {
    font-size: var(--font-medium);
    line-height: 1.9;
    margin-bottom: 1.5rem;
  }

  p:last-child {
    margin-bottom: 0;
  }

  footer {
    text-align: center;
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 1px solid var(--border-colour);
  }

  footer p {
    margin-bottom: 0.75rem;
    font-size: var(--font-small);
  }
</style>