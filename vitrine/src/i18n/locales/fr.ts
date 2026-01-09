export default {
  components: {
    appBar: {
      links: {
        home: 'Accueil',
        about: 'À propos',
        product: 'Produit'
      }
    },
    localeDropdown: {
      en: 'Anglais',
      fr: 'Français'
    },
    pricingSection: {
      discount: 'Économisez {param}%',
      popular: '🔥 Le plus populaire',
      perMonth: 'Par mois/utilisateur'
    }
  },
  pages: {
    home: {
      hero: {
        badge: 'Plateforme souveraine de pentesting',
        title: 'La plateforme collaborative qui révolutionne le pentesting',
        subtitle:
          'Unifiez vos outils, automatisez vos rapports et collaborez en temps réel. ICARUS centralise Exegol et bien plus dans une interface moderne pensée pour les pentesters professionnels.',
        getStarted: 'Démarrer gratuitement',
        learnMore: 'Découvrir la démo',
        features: {
          0: 'Outils intégrés',
          1: 'Collaboration temps réel',
          2: 'Dashboard client'
        }
      },
      problems: {
        badge: 'Les défis du pentesting',
        title: 'Les problèmes que vous rencontrez au quotidien',
        subtitle:
          'Le pentesting moderne exige plus que des compétences techniques. Les pentesters perdent un temps précieux à gérer des outils dispersés.',
        items: {
          0: {
            title: 'Outils dispersés et configuration complexe',
            description:
              'Configuration fastidieuse de Metasploit, Dirb, Burp Suite... Chaque mission nécessite des heures de préparation avant même de commencer le test.'
          },
          1: {
            title: 'Collaboration difficile en équipe',
            description:
              "Partager les découvertes entre pentesters via Slack, emails ou fichiers partagés crée de la confusion et des risques de perte d'information."
          },
          2: {
            title: 'Rapports manuels chronophages',
            description:
              'Rédiger et formater des rapports professionnels prend des jours. Chaque vulnérabilité doit être documentée, capturée et mise en forme manuellement.'
          },
          3: {
            title: 'Manque de transparence client',
            description:
              "Les clients attendent des mises à jour continues, mais communiquer l'avancement sans compromettre la sécurité est un casse-tête permanent."
          }
        }
      },
      solution: {
        badge: 'La solution ICARUS',
        title: 'Une plateforme tout-en-un conçue pour les professionnels',
        subtitle:
          'ICARUS centralise vos outils de pentesting, automatise vos workflows et offre une transparence totale à vos clients, le tout dans une interface moderne et sécurisée.',
        cta: 'Essayer ICARUS',
        features: {
          0: 'Intégration automatique de Exegol',
          1: 'Collaboration en temps réel avec commentaires et notifications',
          2: 'Rapports PDF/HTML générés automatiquement',
          3: 'Dashboard client avec suivi en direct (optionnel)'
        },
        benefits: {
          0: {
            title: 'Souveraineté & Sécurité',
            description:
              'Hébergement souverain français. Vos données sensibles restent sous votre contrôle, avec chiffrement de bout en bout et conformité RGPD.'
          },
          1: {
            title: 'Gain de temps massif',
            description:
              "Économisez jusqu'à 70% du temps passé en configuration, documentation et communication. Focalisez-vous sur ce qui compte : trouver les vulnérabilités."
          },
          2: {
            title: 'Évolutivité & Intégrations',
            description:
              "Architecture modulaire permettant d'ajouter vos propres outils et scripts. API complète pour automatiser vos workflows personnalisés."
          }
        }
      },
      techStack: {
        badge: 'Technologies & Outils',
        title: "Un écosystème complet d'outils de pentesting",
        subtitle:
          'ICARUS intègre nativement les meilleurs outils open-source et commerciaux, avec un framework propriétaire de post-exploitation.',
        tools: {
          0: {
            name: 'Metasploit Framework',
            description:
              'Exploitation automatisée avec notre interface graphique intuitive'
          },
          1: {
            name: 'Suite Exegol complète',
            description:
              'Environnement pré-configuré avec 500+ outils de sécurité'
          },
          2: {
            name: 'Framework post-exploitation',
            description:
              "Notre outil propriétaire pour pivoter et maintenir l'accès"
          },
          3: {
            name: 'Orchestration automatisée',
            description:
              'Gestion intelligente des scans, rapports et notifications'
          },
          4: {
            name: 'Rapports enrichis',
            description:
              'Templates professionnels PDF/HTML avec captures et recommandations'
          },
          5: {
            name: 'Dashboard analytique',
            description:
              'Visualisation en temps réel des vulnérabilités et de la progression'
          }
        }
      },
      useCases: {
        badge: "Cas d'usage",
        title: 'Des équipes qui font confiance à ICARUS',
        subtitle:
          "Du freelance à l'entreprise, découvrez comment ICARUS transforme le quotidien des professionnels de la cybersécurité.",
        cases: {
          0: {
            title: 'Cabinet de conseil en cybersécurité',
            description:
              'Une équipe de 5 consultants réalisant 20+ audits par an. Besoin de coordination efficace et de rapports professionnels rapides.',
            results:
              'Résultat : -60% de temps sur les rapports, collaboration fluide, satisfaction client en hausse.'
          },
          1: {
            title: 'Entreprise avec équipe Red Team interne',
            description:
              "Société du CAC40 avec une équipe de 12 pentesters effectuant des tests continus sur l'infrastructure interne.",
            results:
              'Résultat : Centralisation des données, traçabilité complète, dashboards pour la direction.'
          },
          2: {
            title: 'Pentester indépendant',
            description:
              "Freelance spécialisé en tests d'applications web, cherchant à professionnaliser ses livrables et gagner en efficacité.",
            results:
              'Résultat : Image premium auprès des clients, 40% de projets en plus grâce au temps économisé.'
          },
          3: {
            title: 'Organisme de formation en cybersécurité',
            description:
              'École formant des pentesters juniors, nécessitant un environnement pédagogique moderne et collaboratif.',
            results:
              'Résultat : Étudiants autonomes rapidement, suivi des progressions facilité pour les formateurs.'
          }
        }
      },
      socialProof: {
        badge: 'Témoignages',
        title: 'Ils ont adopté ICARUS',
        subtitle:
          'Rejoignez des centaines de pentesters qui ont transformé leur façon de travailler.',
        stats: {
          0: {
            value: '250+',
            label: 'Pentesters actifs'
          },
          1: {
            value: '1500+',
            label: 'Audits réalisés'
          },
          2: {
            value: '70%',
            label: 'Gain de temps moyen'
          },
          3: {
            value: '98%',
            label: 'Satisfaction client'
          }
        },
        testimonials: {
          0: {
            name: 'Alexandre M.',
            role: 'Pentester Senior, Cabinet Big4',
            content:
              'ICARUS a complètement changé ma façon de travailler. Plus besoin de jongler entre 10 terminaux, tout est centralisé et fluide.'
          },
          1: {
            name: 'Sophie L.',
            role: 'Red Team Lead, Startup SaaS',
            content:
              'La collaboration en temps réel est un game-changer. Mon équipe est 2x plus productive et les clients adorent le dashboard.'
          },
          2: {
            name: 'Thomas R.',
            role: 'Consultant Freelance',
            content:
              "Les rapports automatiques m'ont fait gagner une semaine par mission. Je peux enfin me concentrer sur le pentest, pas l'admin."
          }
        }
      },
      faq: {
        badge: 'FAQ',
        title: 'Questions fréquentes',
        subtitle: 'Tout ce que vous devez savoir sur ICARUS',
        questions: {
          0: {
            question: 'Mes données sont-elles vraiment sécurisées ?',
            answer:
              "Absolument. ICARUS est hébergé en France, avec chiffrement end-to-end, conformité RGPD stricte et option d'hébergement on-premise pour les besoins critiques."
          },
          1: {
            question: 'Puis-je intégrer mes propres outils et scripts ?',
            answer:
              "Oui. ICARUS propose une API complète et un système de plugins permettant d'intégrer vos outils propriétaires et vos workflows personnalisés."
          },
          2: {
            question: "Quelle est la courbe d'apprentissage ?",
            answer:
              "ICARUS est conçu pour les pentesters. Si vous connaissez Metasploit et Burp, vous serez opérationnel en moins d'une heure. Formation et documentation complètes incluses."
          },
          3: {
            question: 'Le dashboard client est-il obligatoire ?',
            answer:
              "Non, c'est optionnel. Vous gardez le contrôle total sur ce que vos clients peuvent voir. Vous pouvez désactiver cette fonctionnalité projet par projet."
          },
          4: {
            question: "Y a-t-il une période d'essai ?",
            answer:
              'Oui, 14 jours gratuits sans carte bancaire. Testez toutes les fonctionnalités sur un projet réel avant de vous engager.'
          },
          5: {
            question: 'Proposez-vous un hébergement on-premise ?',
            answer:
              'Oui, pour les entreprises avec des contraintes de souveraineté strictes, nous proposons une version on-premise avec support dédié et formation.'
          }
        }
      },
      contact: {
        badge: 'Contact',
        title: 'Prêt à transformer votre pentesting ?',
        subtitle:
          'Discutons de vos besoins. Notre équipe vous répond sous 24h pour une démo personnalisée.',
        email: {
          label: 'Email'
        },
        form: {
          name: 'Nom complet',
          namePlaceholder: 'Jean Dupont',
          email: 'Email professionnel',
          emailPlaceholder: 'jean.dupont@entreprise.fr',
          message: 'Message',
          messagePlaceholder:
            'Parlez-nous de votre activité et de vos besoins en pentesting...',
          submit: 'Envoyer le message'
        }
      },
      features: {
        badge: 'Fonctionnalités',
        title: 'Tout ce dont vous avez besoin, réuni en un seul endroit',
        description:
          "ICARUS rassemble les fonctionnalités essentielles pour moderniser votre workflow de pentesting, de l'intégration d'outils à la génération de rapports automatisés.",
        cards: {
          '0': {
            title: 'Outils intégrés nativement',
            description: "Exegol accessibles en un clic depuis l'interface"
          },
          '1': {
            title: 'Collaboration temps réel',
            description:
              'Travaillez en équipe avec commentaires, notifications et synchronisation instantanée'
          },
          '2': {
            title: 'Dashboard client transparent',
            description:
              "Partagez l'avancement en temps réel avec vos clients via un portail sécurisé et personnalisable"
          },
          '3': {
            title: 'Sécurité & Souveraineté',
            description:
              'Hébergement souverain français, chiffrement end-to-end, conformité RGPD totale'
          },
          '4': {
            title: 'Automatisation intelligente',
            description:
              'Rapports PDF/HTML générés automatiquement, alertes configurables, workflows personnalisés'
          },
          '5': {
            title: 'Analytics & Reporting',
            description:
              'Visualisez les vulnérabilités en temps réel avec des dashboards interactifs et des exports professionnels'
          }
        }
      },
      pricing: {
        title: 'Des tarifs transparents pour tous les besoins',
        subtitle:
          'Essai gratuit 14 jours, sans carte bancaire. Annulez à tout moment.',
        frequencies: {
          monthly: 'Mensuel',
          yearly: 'Annuel'
        },
        tiers: {
          starter: {
            name: 'Starter',
            description:
              'Idéal pour les pentesters indépendants ou les petites équipes qui débutent.',
            features: {
              0: "Jusqu'à 3 projets simultanés",
              1: 'Intégration de tous les outils essentiels',
              2: 'Rapports PDF/HTML automatisés',
              3: 'Support par email (48h)',
              4: '10 GB de stockage'
            },
            cta: 'Démarrer gratuitement'
          },
          pro: {
            name: 'Pro',
            description:
              'Pour les pentesters professionnels et les équipes qui veulent aller plus loin.',
            features: {
              0: 'Projets illimités',
              1: 'Collaboration en temps réel',
              2: 'Dashboard client interactif',
              3: 'API complète et webhooks',
              4: 'Support prioritaire (24h)',
              5: '100 GB de stockage',
              6: 'Templates de rapports personnalisés'
            },
            cta: "Commencer l'essai Pro"
          },
          enterprise: {
            name: 'Enterprise',
            price: 'Sur devis',
            description:
              'Solution sur-mesure pour les grandes équipes et entreprises avec besoins spécifiques.',
            features: {
              '0': 'Tout de Pro, plus :',
              '1': 'Hébergement on-premise ou cloud privé',
              '2': 'SSO et intégration Active Directory',
              '3': 'Support dédié 24/7 avec SLA',
              '4': 'Formation et onboarding personnalisés',
              '5': 'Fonctionnalités sur-mesure et développements',
              '6': 'Stockage illimité'
            },
            cta: 'Contactez-nous'
          }
        }
      }
    },
    about: {
      badge: "À propos d'ICARUS",
      title: 'Nous révolutionnons le pentesting professionnel',
      subtitle:
        "Née de la frustration de pentesters expérimentés face à l'inefficacité des outils existants, ICARUS est la plateforme collaborative souveraine qui transforme la manière dont les professionnels de la cybersécurité travaillent.",
      mission: {
        title: 'Notre mission',
        description:
          "ICARUS est né d'un constat simple : les pentesters perdent un temps précieux à jongler entre des dizaines d'outils, à rédiger des rapports manuellement et à coordonner leurs équipes par email ou Slack.",
        detail:
          "Notre mission est de centraliser, automatiser et moderniser le workflow de pentesting pour permettre aux professionnels de se concentrer sur l'essentiel : découvrir et documenter les vulnérabilités. Avec ICARUS, chaque pentester gagne en productivité, en efficacité et en professionnalisme.",
        stats: {
          title: 'ICARUS en chiffres',
          founded: '2025',
          foundedLabel: 'Année de création',
          users: '0',
          usersLabel: 'Pentesters actifs',
          countries: '1',
          countriesLabel: 'Pays représentés'
        }
      },
      valuesTitle: 'Nos valeurs fondamentales',
      valuesSubtitle:
        'Ce qui guide chaque décision que nous prenons pour construire la meilleure plateforme de pentesting.',
      values: {
        0: {
          title: 'Sécurité & Souveraineté',
          description:
            'Hébergement souverain en France, chiffrement end-to-end, zéro compromis sur la confidentialité de vos données. Nous ne vendons ni ne partageons jamais vos informations.'
        },
        1: {
          title: 'Orienté pentesters',
          description:
            'Conçu par des pentesters, pour des pentesters. Chaque fonctionnalité est pensée pour répondre aux besoins réels du terrain, sans fonctionnalités superflues.'
        },
        2: {
          title: 'Innovation continue',
          description:
            "Nous écoutons notre communauté et intégrons régulièrement de nouvelles fonctionnalités. L'API ouverte permet d'intégrer vos propres outils."
        },
        3: {
          title: 'Transparence totale',
          description:
            'Pricing clair, roadmap publique, communication ouverte. Nous croyons en une relation de confiance avec nos utilisateurs.'
        }
      },
      team: {
        title: 'Équipe passionnée, expérience réelle',
        description:
          "Notre équipe est composée de pentesters, développeurs et experts en cybersécurité qui vivent les défis du pentesting au quotidien. Nous construisons l'outil que nous voulons utiliser.",
        contact:
          "Vous souhaitez en savoir plus sur notre équipe ou rejoindre l'aventure ? Contactez-nous à contact@icarus-cyber.fr"
      }
    },
    product: {
      badge: 'Produit',
      title: 'La plateforme complète pour professionnaliser votre pentesting',
      subtitle:
        "Découvrez toutes les fonctionnalités qui font d'ICARUS l'outil indispensable des pentesters modernes.",
      ctaDemo: 'Voir la démo',
      ctaTry: 'Essai gratuit 14 jours',
      benefitsTitle: 'Pourquoi choisir ICARUS ?',
      benefits: {
        0: "Gagnez jusqu'à 70% de temps sur la configuration, la documentation et les rapports",
        1: 'Intégration native de Metasploit, Dirb, Nmap, Exegol et 20+ outils essentiels',
        2: 'Collaboration en temps réel avec votre équipe et vos clients',
        3: 'Sécurité souveraine : hébergement français, chiffrement end-to-end, RGPD',
        4: 'API complète pour automatiser et intégrer vos propres outils'
      },
      featuresTitle: 'Fonctionnalités détaillées',
      featuresSubtitle:
        "Plongez dans les capacités qui font d'ICARUS la référence du pentesting collaboratif.",
      features: {
        0: {
          title: "Suite d'outils intégrés",
          description:
            'Tous vos outils de pentesting centralisés dans une interface unifiée',
          items: {
            0: 'Metasploit Framework avec interface graphique moderne',
            1: 'Exegol pré-configuré avec 500+ outils de sécurité',
            2: 'Nmap, Dirb, Burp Suite, et bien plus intégrés nativement',
            3: 'Framework propriétaire de post-exploitation et pivoting'
          }
        },
        1: {
          title: 'Collaboration équipe avancée',
          description:
            'Travaillez efficacement en équipe avec synchronisation en temps réel',
          items: {
            0: 'Commentaires et annotations sur les vulnérabilités',
            1: 'Notifications intelligentes et alertes personnalisables',
            2: 'Gestion des rôles et permissions (admin, pentester, observateur)',
            3: 'Historique complet des actions et audit trail'
          }
        },
        2: {
          title: 'Rapports professionnels automatisés',
          description:
            'Générez des rapports de qualité premium en quelques clics',
          items: {
            0: 'Templates PDF/HTML personnalisables avec votre branding',
            1: "Captures d'écran automatiques et preuve de concept",
            2: 'Recommandations de remédiation intégrées',
            3: 'Export en Markdown, JSON, ou formats custom via API'
          }
        },
        3: {
          title: 'Dashboard client transparent',
          description: 'Offrez une visibilité totale à vos clients (optionnel)',
          items: {
            0: 'Portail client sécurisé avec accès en lecture seule',
            1: "Suivi en temps réel de l'avancement du pentest",
            2: 'Contrôle granulaire sur ce qui est partagé',
            3: 'Statistiques et graphiques interactifs'
          }
        }
      },
      cta: {
        title: 'Prêt à moderniser votre pentesting ?',
        description:
          'Rejoignez des centaines de pentesters qui ont déjà adopté ICARUS. Essai gratuit 14 jours, sans carte bancaire.',
        button: 'Démarrer gratuitement'
      }
    }
  }
} as const
