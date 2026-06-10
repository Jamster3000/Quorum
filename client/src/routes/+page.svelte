<script lang="ts">
  import Titlebar from '$lib/components/layout/Titlebar.svelte';
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/Footer.svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import {
    IconMessages,
    IconLayoutKanban,
    IconLock,
    IconPhoto,
    IconCode,
    IconUsers,
    IconShieldCheck,
    IconServerBolt,
    IconBrandGithub
  } from '@tabler/icons-svelte';
</script>

<Titlebar />
<Header />

<div class="placeholder-banner">
  Content in the main tag HTML was AI generated as a placeholder and is planned to be replaced with real, accurate content.
</div>

<main class="page">

  <section class="hero">
    <div class="eyebrow">Open source & self-hostable</div>
    <h1>Your community,<br><span class="teal">your rules.</span></h1>
    <p>Chat, collaborate, and build together — without handing your data to anyone. Quorum is open, private, and yours.</p>
    <div class="hero-ctas">
      <Button variant="primary" href="/signup" fontSize="large">Get started free</Button>
      <Button variant="secondary" href="/login" fontSize="large">Log in</Button>
    </div>
    <div class="hero-stats">
      <div class="stat">
        <span class="stat-num teal">100%</span>
        <span class="stat-label">Your data</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat">
        <span class="stat-num teal">Free</span>
        <span class="stat-label">Forever</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat">
        <span class="stat-num teal">MIT</span>
        <span class="stat-label">Licensed</span>
      </div>
    </div>
  </section>

  <section class="features-section">
    <h2>Everything your community needs</h2>
    <p class="section-sub">No plugins, no paywalls. It's all just there.</p>
    <div class="features-grid">
      <div class="feature-card">
        <div class="feature-icon teal-icon"><IconMessages size={22} /></div>
        <h3>Chat & voice</h3>
        <p>Text channels, voice rooms, and direct messages — all in one place, no extra setup.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon violet-icon"><IconLayoutKanban size={22} /></div>
        <h3>Project boards</h3>
        <p>Kanban, Gantt, calendar, and to-do views built directly into every group.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon teal-icon"><IconCode size={22} /></div>
        <h3>Shared notepad</h3>
        <p>A shared file tree and editor for groups that like to build and write things together.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon violet-icon"><IconPhoto size={22} /></div>
        <h3>Photo galleries</h3>
        <p>Dedicated gallery spaces to share and browse images with your group.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon teal-icon"><IconUsers size={22} /></div>
        <h3>Open groups</h3>
        <p>Browse and join communities across any topic — or keep yours completely private.</p>
      </div>
      <div class="feature-card">
        <div class="feature-icon violet-icon"><IconLock size={22} /></div>
        <h3>Privacy first</h3>
        <p>No tracking, no ads, no middleman. Self-host it and own every byte of your data.</p>
      </div>
    </div>
  </section>

  <section class="why-section">
    <div class="why-card">
      <div class="why-icon"><IconShieldCheck size={28} color="var(--primary-colour)" /></div>
      <h3>You own your data</h3>
      <p>When you self-host Quorum, your data never leaves your server. No third party ever touches it.</p>
    </div>
    <div class="why-card featured">
      <div class="why-icon"><IconServerBolt size={28} color="var(--secondary-colour)" /></div>
      <h3>Self-host in minutes</h3>
      <p>A single Docker container is all it takes. No complicated infrastructure, no cloud bills.</p>
    </div>
    <div class="why-card">
      <div class="why-icon"><IconBrandGithub size={28} color="var(--primary-colour)" /></div>
      <h3>Fully open source</h3>
      <p>MIT licensed. Read it, fork it, contribute to it. No black boxes, no hidden behaviour.</p>
    </div>
  </section>

  <section class="cta-section">
    <h2>Ready to take back control?</h2>
    <p>Set up your community in minutes. Free, open, and yours.</p>
    <div class="cta-buttons">
      <Button variant="primary" href="/signup" fontSize="large">Create your account</Button>
      <Button variant="secondary" href="#" fontSize="large">View on GitHub</Button>
    </div>
  </section>
</main>

<Footer />

<style>
  .teal { color: var(--primary-colour); }

  .hero {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 4rem 2rem 3rem;
    max-width: 700px;
    margin: 0 auto;
    gap: 1.25rem;
  }

  .eyebrow {
    display: inline-block;
    background: color-mix(in srgb, var(--secondary-colour) 15%, transparent);
    border: 1px solid color-mix(in srgb, var(--secondary-colour) 40%, transparent);
    color: var(--secondary-colour);
    font-size: 13px;
    font-weight: 700;
    padding: 4px 14px;
    border-radius: 999px;
  }

  h1 {
    font-size: 52px;
    font-weight: 700;
    line-height: 1.15;
    color: var(--text-colour);
  }

  .hero p {
    font-size: 17px;
    color: var(--text-muted-colour);
    line-height: 1.7;
    max-width: 520px;
  }

  .hero-ctas {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    justify-content: center;
  }

  .hero-stats {
    display: flex;
    align-items: center;
    gap: 2rem;
    margin-top: 1rem;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
  }

  .stat-num {
    font-size: 22px;
    font-weight: 700;
  }

  .stat-label {
    font-size: 12px;
    color: var(--text-muted-colour);
    font-weight: 600;
  }

  .stat-divider {
    width: 1px;
    height: 32px;
    background: var(--border-colour);
  }

  .features-section {
    padding: 4rem 4rem 3rem;
    max-width: 1100px;
    margin: 0 auto;
    text-align: center;
  }

  .features-section h2 {
    font-size: 30px;
    font-weight: 700;
    color: var(--text-colour);
    margin-bottom: 0.5rem;
  }

  .section-sub {
    font-size: 15px;
    color: var(--text-muted-colour);
    margin-bottom: 2.5rem;
  }

  .features-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1.25rem;
    text-align: left;
  }

  .feature-card {
    background: var(--card-colour);
    border: 1px solid var(--border-colour);
    border-radius: 16px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .feature-card:hover {
    border-color: var(--primary-colour);
    box-shadow: 0 0 24px color-mix(in srgb, var(--primary-colour) 12%, transparent);
  }

  .feature-icon {
    width: 42px;
    height: 42px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .teal-icon {
    background: color-mix(in srgb, var(--primary-colour) 15%, transparent);
    color: var(--primary-colour);
  }

  .violet-icon {
    background: color-mix(in srgb, var(--secondary-colour) 15%, transparent);
    color: var(--secondary-colour);
  }

  .feature-card h3 {
    font-size: 15px;
    font-weight: 700;
    color: var(--text-colour);
  }

  .feature-card p {
    font-size: 13px;
    color: var(--text-muted-colour);
    line-height: 1.6;
  }

  .why-section {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1.25rem;
    padding: 2rem 4rem 4rem;
    max-width: 1100px;
    margin: 0 auto;
  }

  .why-card {
    background: var(--card-colour);
    border: 1px solid var(--border-colour);
    border-radius: 16px;
    padding: 1.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .why-card.featured {
    border-color: color-mix(in srgb, var(--secondary-colour) 50%, transparent);
    box-shadow: 0 0 28px color-mix(in srgb, var(--secondary-colour) 12%, transparent);
  }

  .why-icon {
    width: 48px;
    height: 48px;
    border-radius: 14px;
    background: var(--background-colour);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .why-card h3 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text-colour);
  }

  .why-card p {
    font-size: 13px;
    color: var(--text-muted-colour);
    line-height: 1.6;
  }

  .cta-section {
    text-align: center;
    padding: 3rem 2rem 4rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    border-top: 1px solid var(--border-colour);
  }

  .cta-section h2 {
    font-size: 30px;
    font-weight: 700;
    color: var(--text-colour);
  }

  .cta-section p {
    font-size: 15px;
    color: var(--text-muted-colour);
  }

  .cta-buttons {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    justify-content: center;
    margin-top: 0.5rem;
  }

  .placeholder-banner {
    background: color-mix(in srgb, var(--warning-colour) 12%, transparent);
    border-bottom: 1px solid color-mix(in srgb, var(--warning-colour) 30%, transparent);
    color: var(--warning-colour);
    font-size: 13px;
    font-weight: 600;
    text-align: center;
    padding: 8px 2rem;
  }
</style>