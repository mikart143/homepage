<script lang="ts">
  import GlassSection from './GlassSection.svelte';
</script>

<section id="contact" class="mb-20 w-full max-w-4xl" aria-label="Contact">
  <GlassSection class="p-10">
    <h2 class="mb-4 text-center text-4xl font-bold text-white focus:rounded focus:outline-2">
      Get in Touch
    </h2>
    <p class="mb-10 text-center text-white/70">
      Let's discuss your next project or collaboration opportunity
    </p>

    <div class="rounded-xl border border-white/20 bg-white/5 p-8 backdrop-blur-lg">
      <form
        action="https://submit-form.com/A9BYhfqrW"
        method="POST"
        class="space-y-6"
        aria-label="Contact form"
      >
        <div class="form-group">
          <label for="name" class="mb-2 block text-sm font-medium text-white/70">
            Name
            <span class="text-red-400" aria-label="required">*</span>
          </label>
          <input
            type="text"
            id="name"
            name="name"
            placeholder="Your name"
            required
            aria-required="true"
            class="w-full rounded-lg border border-white/20 bg-white/5 px-4 py-3 text-white placeholder-white/40 transition-colors focus:outline-2 focus:outline-offset-2 focus:outline-blue-400"
          />
        </div>
        <div class="form-group">
          <label for="email" class="mb-2 block text-sm font-medium text-white/70">
            Email
            <span class="text-red-400" aria-label="required">*</span>
          </label>
          <input
            type="email"
            id="email"
            name="email"
            placeholder="your.email@example.com"
            required
            aria-required="true"
            class="w-full rounded-lg border border-white/20 bg-white/5 px-4 py-3 text-white placeholder-white/40 transition-colors focus:outline-2 focus:outline-offset-2 focus:outline-blue-400"
          />
        </div>
        <div class="form-group">
          <label for="message" class="mb-2 block text-sm font-medium text-white/70">
            Message
            <span class="text-red-400" aria-label="required">*</span>
          </label>
          <textarea
            id="message"
            name="message"
            placeholder="Tell me about your project or inquiry..."
            rows="5"
            required
            aria-required="true"
            class="w-full resize-none rounded-lg border border-white/20 bg-white/5 px-4 py-3 text-white placeholder-white/40 transition-colors focus:outline-2 focus:outline-offset-2 focus:outline-blue-400"
          ></textarea>
        </div>
        <button
          type="submit"
          class="w-full rounded-lg bg-linear-to-r from-blue-500 to-purple-600 py-3 font-semibold text-white shadow-lg transition-transform hover:scale-[1.02] focus:outline-2 focus:outline-offset-2 focus:outline-blue-400"
          aria-label="Send your message"
        >
          Send Message
        </button>
      </form>
    </div>
  </GlassSection>
</section>
