import { ContactSection } from '@/components/sections/contact-section'
import { FAQSection } from '@/components/sections/faq-section'
import { FeaturesSection } from '@/components/sections/features-section'
import { HeroSection } from '@/components/sections/hero-section'
import { PricingSection } from '@/components/sections/pricing-section'
import { ProblemsSection } from '@/components/sections/problems-section'
import { SolutionSection } from '@/components/sections/solution-section'

export default function HomePage() {
  return (
    <>
      <HeroSection />
      <ProblemsSection />
      <SolutionSection />
      <FeaturesSection />
      <PricingSection />
      <FAQSection />
      <ContactSection />
    </>
  )
}
