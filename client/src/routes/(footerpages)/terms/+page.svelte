<script lang="ts">
  import Header from '$lib/components/app/Header.svelte';
  import Footer from '$lib/components/app/Footer.svelte';
  import Auth from '$lib/utils/Auth.svelte';
  import Card from '$lib/components/ui/Card.svelte';
  import { IconAlertCircle, IconShield, IconUsers, IconLock, IconFileText } from '@tabler/icons-svelte-runes';
</script>

<Auth redirectTo="/home" shouldBeLoggedIn={false} />
<Header />

<main class="page">
  <Card padding="xlarge" width="50%" variant="gradient">
    <article>
      <h1>Terms of Service</h1>

      <section>
        <h2>1. Acceptance of Terms</h2>
        <p>
          By accessing, downloading, installing, or using the Quorum application (the "Application")
          and its associated services operated by Jamster (collectively, the "Service"), you agree to
          be bound by these Terms of Service. If you do not agree to these terms, you may not use the
          Application. Your continued use of the Application constitutes your acceptance of these terms.
        </p>
      </section>

      <section>
        <h2>2. Use License</h2>
        <p>
          The Application is licensed under the AGPL-3.0 license. You are granted a limited, non-exclusive,
          non-transferable, and revocable license to use the Application in accordance with the terms of
          the AGPL-3.0 license. You may not sublicense, sell, rent, lease, transfer, assign, or otherwise
          dispose of the Application except as expressly permitted by the license.
        </p>
        <p>
          For open-source modifications and contributions, you must comply with all terms of the AGPL-3.0
          license, including the requirement to share modifications and maintain the same license.
        </p>
      </section>

      <section>
        <h2>3. User Accounts</h2>
        <p>
          To use certain features of the Application, you may be required to create an account. You are
          responsible for maintaining the confidentiality of your account credentials and password. You
          agree to accept responsibility for all activities that occur under your account. You must notify
          us immediately if you become aware of any unauthorized use of your account.
        </p>
        <p>
          You warrant that all information provided during account creation is accurate, current, and
          complete. You agree not to use false or misleading information when creating your account.
        </p>
      </section>

      <section>
        <h2>4. User Conduct</h2>
        <p>
          You agree not to use the Application in any manner that:
        </p>
        <p>
          (a) Violates any applicable law, regulation, or third-party right;<br/>
          (b) Infringes on any intellectual property rights;<br/>
          (c) Harasses, threatens, defames, or abuses others;<br/>
          (d) Transmits malware, viruses, or harmful code;<br/>
          (e) Attempts to gain unauthorized access to systems or accounts;<br/>
          (f) Disrupts the normal operation of the Application or its services;<br/>
          (g) Impersonates any person or entity;<br/>
          (h) Collects or tracks personal information of others without consent;<br/>
          (i) Sends spam, unsolicited messages, or commercial advertising;<br/>
          (j) Engages in any form of abuse, exploitation, or illegal activity.
        </p>
      </section>

      <section>
        <h2>5. Content and Moderation</h2>
        <p>
          Jamster reserves the right to moderate, filter, or remove content that violates these Terms
          of Service or applicable law. The Application may employ automated content filtering and
          detection systems, including encrypted analysis of messages, to maintain community standards.
        </p>
        <p>
          You retain ownership of any content you create or upload. By using the Application, you grant
          Jamster a license to use, display, and store your content for the purpose of providing the
          Service to you and other authorized users.
        </p>
      </section>

      <section>
        <h2>6. Privacy and Data</h2>
        <p>
          Your use of the Application is also governed by our Privacy Policy. Please review the Privacy
          Policy to understand our practices regarding data collection, processing, and storage.
        </p>
        <p>
          If you choose to self-host the Application, you are responsible for compliance with all
          applicable data protection laws and regulations in your jurisdiction.
        </p>
      </section>

      <section>
        <h2>7. Disclaimer of Warranties</h2>
        <p>
          THE APPLICATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
          BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, OR
          NON-INFRINGEMENT. JAMSTER DOES NOT WARRANT THAT THE APPLICATION WILL BE UNINTERRUPTED, ERROR-FREE,
          OR SECURE.
        </p>
        <p>
          You assume all risks associated with your use of the Application, including the risk of data loss,
          service interruption, or security breach.
        </p>
      </section>

      <section>
        <h2>8. Limitation of Liability</h2>
        <p>
          TO THE MAXIMUM EXTENT PERMITTED BY LAW, JAMSTER SHALL NOT BE LIABLE FOR ANY INDIRECT, INCIDENTAL,
          SPECIAL, CONSEQUENTIAL, OR PUNITIVE DAMAGES ARISING OUT OF OR IN CONNECTION WITH YOUR USE OF OR
          INABILITY TO USE THE APPLICATION, INCLUDING BUT NOT LIMITED TO DAMAGES FOR LOSS OF DATA, PROFITS,
          GOODWILL, OR OTHER INTANGIBLE LOSSES, EVEN IF JAMSTER HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
        </p>
      </section>

      <section>
        <h2>9. Indemnification</h2>
        <p>
          You agree to indemnify, defend, and hold harmless Jamster, its officers, employees, and agents
          from any and all claims, damages, liabilities, costs, and expenses (including reasonable attorneys'
          fees) arising from or related to your violation of these Terms of Service, your use of the Application,
          or your violation of any law or the rights of any third party.
        </p>
      </section>

      <section>
        <h2>10. Termination</h2>
        <p>
          Jamster may terminate or suspend your account and access to the Application at any time, without
          prior notice or cause, for any reason including but not limited to: violation of these Terms of
          Service, illegal activity, or abuse of the Service.
        </p>
        <p>
          Upon termination, your right to use the Application immediately ceases. Any provisions that by
          their nature should survive termination shall remain in effect.
        </p>
      </section>

      <section>
        <h2>11. Changes to Terms</h2>
        <p>
          Jamster may modify these Terms of Service at any time. The most current version will be posted on
          this page with an updated effective date. Your continued use of the Application after any changes
          constitutes your acceptance of the modified terms. It is your responsibility to review these terms
          regularly for updates.
        </p>
      </section>

      <section>
        <h2>12. Governing Law</h2>
        <p>
          These Terms of Service are governed by and construed in accordance with the laws of the jurisdiction
          in which Jamster operates, without regard to its conflict of law provisions. You agree to submit to
          the exclusive jurisdiction of the courts in that jurisdiction.
        </p>
      </section>

      <section>
        <h2>13. Severability</h2>
        <p>
          If any provision of these Terms of Service is found to be invalid or unenforceable by a court of
          competent jurisdiction, that provision shall be severed, and the remaining provisions shall remain
          in full force and effect to the maximum extent permitted by law.
        </p>
      </section>

      <section>
        <h2>14. Entire Agreement</h2>
        <p>
          These Terms of Service, together with the Privacy Policy, constitute the entire agreement between
          you and Jamster regarding your use of the Application and supersede all prior and contemporaneous
          agreements, understandings, and communications, whether written or oral.
        </p>
      </section>

      <section>
        <h2>15. Contact Us</h2>
        <p>
          If you have any questions regarding these Terms of Service or the Application, please contact
          Jamster via email at <a href="mailto:[Email Address Placeholder]">[Email Address Placeholder]</a>.
        </p>
      </section>

      <footer>
        <p><small>Effective as of 2026-07-30</small></p>
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