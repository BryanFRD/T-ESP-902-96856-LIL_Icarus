export default {
  components: {
    appBar: {
      links: {
        home: 'Home',
        about: 'About',
        product: 'Product'
      }
    },
    localeDropdown: {
      en: 'English',
      fr: 'French'
    },
    pricingSection: {
      discount: 'Save {param}%',
      popular: '🔥 Most Popular',
      perMonth: 'Per month/user'
    }
  },
  pages: {
    home: {
      hero: {
        badge: 'Sovereign pentesting platform',
        title: 'The collaborative platform revolutionizing pentesting',
        subtitle:
          'Unify your tools, automate reports, and collaborate in real-time. ICARUS centralizes Exegol and more in a modern interface built for professional pentesters.',
        getStarted: 'Start for free',
        learnMore: 'Watch demo',
        features: {
          0: 'Integrated tools',
          1: 'Real-time collaboration',
          2: 'Client dashboard'
        }
      },
      problems: {
        badge: 'Pentesting challenges',
        title: 'Problems you face every day',
        subtitle:
          'Modern pentesting requires more than technical skills. Pentesters waste precious time managing scattered tools.',
        items: {
          0: {
            title: 'Scattered tools and complex setup',
            description:
              'Tedious configuration of Metasploit, Dirb, Burp Suite... Each mission requires hours of preparation before even starting the test.'
          },
          1: {
            title: 'Difficult team collaboration',
            description:
              'Sharing findings between pentesters via Slack, emails or shared files creates confusion and risks of information loss.'
          },
          2: {
            title: 'Time-consuming manual reports',
            description:
              'Writing and formatting professional reports takes days. Each vulnerability must be documented, captured and formatted manually.'
          },
          3: {
            title: 'Lack of client transparency',
            description:
              'Clients expect continuous updates, but communicating progress without compromising security is a permanent headache.'
          }
        }
      },
      solution: {
        badge: 'The ICARUS solution',
        title: 'An all-in-one platform built for professionals',
        subtitle:
          'ICARUS centralizes your pentesting tools, automates your workflows and offers total transparency to your clients, all in a modern and secure interface.',
        cta: 'Try ICARUS',
        features: {
          0: 'Automatic integration of Exegol',
          1: 'Real-time collaboration with comments and notifications',
          2: 'PDF/HTML reports generated automatically',
          3: 'Client dashboard with live tracking (optional)'
        },
        benefits: {
          0: {
            title: 'Sovereignty & Security',
            description:
              'French sovereign hosting. Your sensitive data remains under your control, with end-to-end encryption and GDPR compliance.'
          },
          1: {
            title: 'Massive time savings',
            description:
              'Save up to 70% of time spent on configuration, documentation and communication. Focus on what matters: finding vulnerabilities.'
          },
          2: {
            title: 'Scalability & Integrations',
            description:
              'Modular architecture allowing you to add your own tools and scripts. Complete API to automate your custom workflows.'
          }
        }
      },
      techStack: {
        badge: 'Technologies & Tools',
        title: 'A complete ecosystem of pentesting tools',
        subtitle:
          'ICARUS natively integrates the best open-source and commercial tools, with a proprietary post-exploitation framework.',
        tools: {
          0: {
            name: 'Metasploit Framework',
            description:
              'Automated exploitation with our intuitive graphical interface'
          },
          1: {
            name: 'Complete Exegol Suite',
            description: 'Pre-configured environment with 500+ security tools'
          },
          2: {
            name: 'Post-exploitation framework',
            description: 'Our proprietary tool to pivot and maintain access'
          },
          3: {
            name: 'Automated orchestration',
            description:
              'Intelligent management of scans, reports and notifications'
          },
          4: {
            name: 'Enhanced reports',
            description:
              'Professional PDF/HTML templates with screenshots and recommendations'
          },
          5: {
            name: 'Analytics dashboard',
            description:
              'Real-time visualization of vulnerabilities and progress'
          }
        }
      },
      useCases: {
        badge: 'Use cases',
        title: 'Teams that trust ICARUS',
        subtitle:
          'From freelance to enterprise, discover how ICARUS transforms the daily life of cybersecurity professionals.',
        cases: {
          0: {
            title: 'Cybersecurity consulting firm',
            description:
              'Team of 5 consultants conducting 20+ audits per year. Need for efficient coordination and fast professional reports.',
            results:
              'Result: -60% time on reports, smooth collaboration, increased client satisfaction.'
          },
          1: {
            title: 'Company with internal Red Team',
            description:
              'CAC40 company with a team of 12 pentesters performing continuous tests on internal infrastructure.',
            results:
              'Result: Data centralization, complete traceability, dashboards for management.'
          },
          2: {
            title: 'Independent pentester',
            description:
              'Freelancer specializing in web application testing, looking to professionalize deliverables and gain efficiency.',
            results:
              'Result: Premium image with clients, 40% more projects thanks to time saved.'
          },
          3: {
            title: 'Cybersecurity training organization',
            description:
              'School training junior pentesters, requiring a modern and collaborative educational environment.',
            results:
              'Result: Autonomous students quickly, easier progress tracking for trainers.'
          }
        }
      },
      socialProof: {
        badge: 'Testimonials',
        title: 'They adopted ICARUS',
        subtitle:
          'Join hundreds of pentesters who have transformed their way of working.',
        stats: {
          0: {
            value: '250+',
            label: 'Active pentesters'
          },
          1: {
            value: '1500+',
            label: 'Completed audits'
          },
          2: {
            value: '70%',
            label: 'Average time saved'
          },
          3: {
            value: '98%',
            label: 'Client satisfaction'
          }
        },
        testimonials: {
          0: {
            name: 'Alexandre M.',
            role: 'Senior Pentester, Big4 Firm',
            content:
              'ICARUS completely changed the way I work. No more juggling between 10 terminals, everything is centralized and smooth.'
          },
          1: {
            name: 'Sophie L.',
            role: 'Red Team Lead, SaaS Startup',
            content:
              'Real-time collaboration is a game-changer. My team is 2x more productive and clients love the dashboard.'
          },
          2: {
            name: 'Thomas R.',
            role: 'Freelance Consultant',
            content:
              'Automatic reports saved me a week per mission. I can finally focus on pentesting, not admin work.'
          }
        }
      },
      faq: {
        badge: 'FAQ',
        title: 'Frequently asked questions',
        subtitle: 'Everything you need to know about ICARUS',
        questions: {
          0: {
            question: 'Is my data really secure?',
            answer:
              'Absolutely. ICARUS is hosted in France, with end-to-end encryption, strict GDPR compliance and on-premise hosting option for critical needs.'
          },
          1: {
            question: 'Can I integrate my own tools and scripts?',
            answer:
              'Yes. ICARUS offers a complete API and plugin system allowing you to integrate your proprietary tools and custom workflows.'
          },
          2: {
            question: 'What is the learning curve?',
            answer:
              "ICARUS is built for pentesters. If you know Metasploit and Burp, you'll be operational in less than an hour. Complete training and documentation included."
          },
          3: {
            question: 'Is the client dashboard mandatory?',
            answer:
              "No, it's optional. You keep total control over what your clients can see. You can disable this feature on a project-by-project basis."
          },
          4: {
            question: 'Is there a trial period?',
            answer:
              'Yes, 14 days free without credit card. Test all features on a real project before committing.'
          },
          5: {
            question: 'Do you offer on-premise hosting?',
            answer:
              'Yes, for companies with strict sovereignty constraints, we offer an on-premise version with dedicated support and training.'
          }
        }
      },
      contact: {
        badge: 'Contact',
        title: 'Ready to transform your pentesting?',
        subtitle:
          "Let's discuss your needs. Our team responds within 24h for a personalized demo.",
        email: {
          label: 'Email'
        },
        form: {
          name: 'Full name',
          namePlaceholder: 'John Doe',
          email: 'Professional email',
          emailPlaceholder: 'john.doe@company.com',
          message: 'Message',
          messagePlaceholder:
            'Tell us about your activity and your pentesting needs...',
          submit: 'Send message'
        }
      },
      features: {
        badge: 'Features',
        title: 'Everything you need, in one place',
        description:
          'ICARUS brings together essential features to modernize your pentesting workflow, from tool integration to automated report generation.',
        cards: {
          '0': {
            title: 'Natively integrated tools',
            description: 'Exegol accessible with one click from the interface'
          },
          '1': {
            title: 'Real-time collaboration',
            description:
              'Work as a team with comments, notifications and instant synchronization'
          },
          '2': {
            title: 'Transparent client dashboard',
            description:
              'Share progress in real-time with your clients via a secure and customizable portal'
          },
          '3': {
            title: 'Security & Sovereignty',
            description:
              'French sovereign hosting, end-to-end encryption, full GDPR compliance'
          },
          '4': {
            title: 'Intelligent automation',
            description:
              'PDF/HTML reports generated automatically, configurable alerts, custom workflows'
          },
          '5': {
            title: 'Analytics & Reporting',
            description:
              'Visualize vulnerabilities in real-time with interactive dashboards and professional exports'
          }
        }
      },
      pricing: {
        title: 'Transparent pricing for all needs',
        subtitle: '14-day free trial, no credit card. Cancel anytime.',
        frequencies: {
          monthly: 'Monthly',
          yearly: 'Yearly'
        },
        tiers: {
          starter: {
            name: 'Starter',
            description:
              'Perfect for independent pentesters or small teams getting started',
            features: {
              '0': 'Up to 3 concurrent projects',
              '1': 'Integration of all essential tools',
              '2': 'Automated PDF/HTML reports',
              '3': 'Email support (48h)',
              '4': '10 GB storage'
            },
            cta: 'Start for free'
          },
          pro: {
            name: 'Pro',
            description:
              'For professional pentesters and teams who want to go further',
            features: {
              '0': 'Unlimited projects',
              '1': 'Real-time collaboration',
              '2': 'Interactive client dashboard',
              '3': 'Full API and webhooks',
              '4': 'Priority support (24h)',
              '5': '100 GB storage',
              '6': 'Custom report templates'
            },
            cta: 'Start Pro trial'
          },
          enterprise: {
            name: 'Enterprise',
            price: 'On request',
            description:
              'Custom solution for large teams and companies with specific needs',
            features: {
              '0': 'Everything in Pro, plus:',
              '1': 'On-premise or private cloud hosting',
              '2': 'SSO and Active Directory integration',
              '3': 'Dedicated 24/7 support with SLA',
              '4': 'Custom training and onboarding',
              '5': 'Custom features and development',
              '6': 'Unlimited storage'
            },
            cta: 'Contact us'
          }
        }
      }
    },
    about: {
      badge: 'About ICARUS',
      title: 'We are revolutionizing professional pentesting',
      subtitle:
        'Born from the frustration of experienced pentesters facing the inefficiency of existing tools, ICARUS is the sovereign collaborative platform that transforms how cybersecurity professionals work.',
      mission: {
        title: 'Our mission',
        description:
          'ICARUS was born from a simple observation: pentesters waste precious time juggling dozens of tools, writing reports manually and coordinating their teams via email or Slack.',
        detail:
          'Our mission is to centralize, automate and modernize the pentesting workflow to allow professionals to focus on the essential: discovering and documenting vulnerabilities. With ICARUS, every pentester gains in productivity, efficiency and professionalism.',
        stats: {
          title: 'ICARUS in numbers',
          founded: '2025',
          foundedLabel: 'Year founded',
          users: '0',
          usersLabel: 'Active pentesters',
          countries: '1',
          countriesLabel: 'Countries represented'
        }
      },
      valuesTitle: 'Our core values',
      valuesSubtitle:
        'What guides every decision we make to build the best pentesting platform.',
      values: {
        0: {
          title: 'Security & Sovereignty',
          description:
            'Sovereign hosting in France, end-to-end encryption, zero compromise on your data confidentiality. We never sell or share your information.'
        },
        1: {
          title: 'Pentester-oriented',
          description:
            'Designed by pentesters, for pentesters. Every feature is designed to meet real field needs, without superfluous features.'
        },
        2: {
          title: 'Continuous innovation',
          description:
            'We listen to our community and regularly integrate new features. The open API allows you to integrate your own tools.'
        },
        3: {
          title: 'Total transparency',
          description:
            'Clear pricing, public roadmap, open communication. We believe in a relationship of trust with our users.'
        }
      },
      team: {
        title: 'Passionate team, real experience',
        description:
          'Our team is composed of pentesters, developers and cybersecurity experts who live the challenges of pentesting every day. We are building the tool we want to use.',
        contact:
          'Want to learn more about our team or join the adventure? Contact us at contact@icarus-cyber.fr'
      }
    },
    product: {
      badge: 'Product',
      title: 'The complete platform to professionalize your pentesting',
      subtitle:
        'Discover all the features that make ICARUS the essential tool for modern pentesters.',
      ctaDemo: 'Watch demo',
      ctaTry: '14-day free trial',
      benefitsTitle: 'Why choose ICARUS?',
      benefits: {
        0: 'Save up to 70% of time on configuration, documentation and reports',
        1: 'Native integration of Exegol and 20+ essential tools',
        2: 'Real-time collaboration with your team and clients',
        3: 'Sovereign security: French hosting, end-to-end encryption, GDPR',
        4: 'Complete API to automate and integrate your own tools'
      },
      featuresTitle: 'Detailed features',
      featuresSubtitle:
        'Dive into the capabilities that make ICARUS the reference for collaborative pentesting.',
      features: {
        0: {
          title: 'Integrated tool suite',
          description:
            'All your pentesting tools centralized in a unified interface',
          items: {
            0: 'Metasploit Framework with modern graphical interface',
            1: 'Pre-configured Exegol with 500+ security tools',
            2: 'Nmap, Dirb, Burp Suite, and more natively integrated',
            3: 'Proprietary post-exploitation and pivoting framework'
          }
        },
        1: {
          title: 'Advanced team collaboration',
          description:
            'Work efficiently as a team with real-time synchronization',
          items: {
            0: 'Comments and annotations on vulnerabilities',
            1: 'Smart notifications and customizable alerts',
            2: 'Role and permission management (admin, pentester, observer)',
            3: 'Complete action history and audit trail'
          }
        },
        2: {
          title: 'Automated professional reports',
          description: 'Generate premium quality reports in a few clicks',
          items: {
            0: 'Customizable PDF/HTML templates with your branding',
            1: 'Automatic screenshots and proof of concept',
            2: 'Integrated remediation recommendations',
            3: 'Export to Markdown, JSON, or custom formats via API'
          }
        },
        3: {
          title: 'Transparent client dashboard',
          description: 'Offer total visibility to your clients (optional)',
          items: {
            0: 'Secure client portal with read-only access',
            1: 'Real-time tracking of pentest progress',
            2: 'Granular control over what is shared',
            3: 'Interactive statistics and charts'
          }
        }
      },
      cta: {
        title: 'Ready to modernize your pentesting?',
        description:
          'Join hundreds of pentesters who have already adopted ICARUS. 14-day free trial, no credit card required.',
        button: 'Start for free'
      }
    }
  }
} as const
