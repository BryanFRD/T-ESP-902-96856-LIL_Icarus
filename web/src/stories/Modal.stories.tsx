import type { Meta, StoryObj } from '@storybook/react'
import { useState } from 'react'
import { action } from 'storybook/actions'

import Button from '@/components/ui/button/Button.tsx'
import Modal from '@/components/ui/modal/Modal.tsx'

const mockFn = (name?: string) => action(`[Storybook] ${name ?? 'action'}`)

const meta: Meta<typeof Modal> = {
  title: 'UI/Modal',
  component: Modal,
  parameters: {
    layout: 'fullscreen',
    docs: {
      story: {
        height: '500px'
      }
    }
  },
  tags: ['autodocs'],
  argTypes: {
    size: {
      control: { type: 'select' },
      options: ['sm', 'md', 'lg', 'xl'],
      description: 'Taille de la modal'
    },
    isOpen: {
      control: { type: 'boolean' },
      description: "État d'ouverture de la modal"
    },
    title: {
      control: { type: 'text' },
      description: 'Titre de la modal'
    },
    closeButtonLabel: {
      control: { type: 'text' },
      description: "Label du bouton de fermeture pour l'accessibilité"
    },
    disableOutsideClick: {
      control: { type: 'boolean' },
      description: "Désactiver la fermeture au clic à l'extérieur"
    },
    disableEscapeKey: {
      control: { type: 'boolean' },
      description: 'Désactiver la fermeture avec la touche Escape'
    }
  },
  args: {
    onClose: mockFn('onClose'),
    closeButtonLabel: 'Fermer'
  }
}

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    isOpen: true,
    title: 'Modal par défaut',
    children: (
      <div className='space-y-4'>
        <p>Ceci est le contenu de la modal par défaut.</p>
        <p>
          Vous pouvez utiliser les contrôles Storybook pour modifier les
          propriétés.
        </p>
        <div className='rounded bg-gray-100 p-4 dark:bg-gray-800'>
          <p className='text-sm'>
            Testez les différentes tailles et options dans les contrôles.
          </p>
        </div>
      </div>
    )
  }
}

export const WithFooter: Story = {
  args: {
    isOpen: true,
    title: 'Modal avec footer',
    children: (
      <div className='space-y-4'>
        <p>
          Cette modal contient un footer personnalisé avec des boutons
          d&apos;action.
        </p>
        <p>Le footer reste collé en bas même avec du contenu défilant.</p>
      </div>
    ),
    footer: (
      <div className='flex justify-end gap-3'>
        <Button variant='ghost' onClick={mockFn('cancel-clicked')}>
          Annuler
        </Button>
        <Button variant='primary' onClick={mockFn('confirm-clicked')}>
          Confirmer
        </Button>
      </div>
    )
  }
}

export const Interactive: Story = {
  render: args => {
    const [isOpen, setIsOpen] = useState(false)

    return (
      <div className='flex min-h-screen items-center justify-center bg-gray-50 p-8 dark:bg-gray-900'>
        <Button onClick={() => setIsOpen(true)} variant='primary'>
          Ouvrir la modal
        </Button>

        <Modal
          {...args}
          isOpen={isOpen}
          onClose={() => {
            setIsOpen(false)
          }}
          title='Modal interactive'
          footer={
            <div className='flex justify-end gap-3'>
              <Button onClick={() => setIsOpen(false)} variant='ghost'>
                Annuler
              </Button>
              <Button onClick={() => setIsOpen(false)} variant='primary'>
                Valider
              </Button>
            </div>
          }
        >
          <div className='space-y-4'>
            <p>
              Cette modal peut être ouverte et fermée de manière interactive.
            </p>
            <div>
              <span className='mb-2 block text-sm font-medium'>
                Champ exemple
              </span>
              <input
                type='text'
                placeholder='Tapez quelque chose...'
                className='w-full rounded-md border border-gray-300 px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white'
              />
            </div>
          </div>
        </Modal>
      </div>
    )
  }
}

export const AllSizes: Story = {
  render: args => {
    const [openModal, setOpenModal] = useState<string | null>(null)
    const sizes = ['sm', 'md', 'lg', 'xl'] as const

    return (
      <div className='min-h-screen bg-gray-50 p-8 dark:bg-gray-900'>
        <div className='mx-auto max-w-4xl space-y-6'>
          <h3 className='text-xl font-bold'>Testez les différentes tailles</h3>
          <div className='flex flex-wrap gap-4'>
            {sizes.map(size => (
              <Button
                key={size}
                onClick={() => setOpenModal(size)}
                variant='primary'
              >
                Modal {size.toUpperCase()}
              </Button>
            ))}
          </div>

          {sizes.map(size => (
            <Modal
              key={size}
              {...args}
              isOpen={openModal === size}
              onClose={() => setOpenModal(null)}
              title={`Modal ${size.toUpperCase()}`}
              size={size}
            >
              <div className='space-y-4'>
                <p>
                  Ceci est une modal de taille <strong>{size}</strong>. Le
                  contenu s&apos;adapte à la largeur définie.
                </p>
                <div className='grid grid-cols-2 gap-4'>
                  <div className='rounded bg-blue-50 p-3 dark:bg-blue-900/20'>
                    <h5 className='font-medium'>Largeur max</h5>
                    <p className='text-sm text-gray-600 dark:text-gray-400'>
                      {size === 'sm' && '384px (24rem)'}
                      {size === 'md' && '448px (28rem)'}
                      {size === 'lg' && '512px (32rem)'}
                      {size === 'xl' && '576px (36rem)'}
                    </p>
                  </div>
                  <div className='rounded bg-green-50 p-3 dark:bg-green-900/20'>
                    <h5 className='font-medium'>Usage</h5>
                    <p className='text-sm text-gray-600 dark:text-gray-400'>
                      {size === 'sm' && 'Confirmations simples'}
                      {size === 'md' && 'Formulaires courts'}
                      {size === 'lg' && 'Contenu détaillé'}
                      {size === 'xl' && 'Formulaires complexes'}
                    </p>
                  </div>
                </div>
              </div>
            </Modal>
          ))}
        </div>
      </div>
    )
  }
}

export const LongContent: Story = {
  args: {
    isOpen: true,
    title: 'Modal avec contenu long',
    size: 'lg',
    children: (
      <div className='space-y-6'>
        <div className='rounded-lg bg-yellow-50 p-4 dark:bg-yellow-900/20'>
          <p className='text-sm'>
            <strong>Test du scroll :</strong> Cette modal contient beaucoup de
            contenu pour tester le comportement de défilement. Le header et
            footer restent fixes.
          </p>
        </div>

        {Array.from({ length: 12 }, (_, i) => (
          <div
            key={i}
            className='rounded-lg border border-gray-200 p-4 dark:border-gray-700'
          >
            <h4 className='mb-2 text-lg font-medium'>Section {i + 1}</h4>
            <p className='mb-3 text-gray-600 dark:text-gray-400'>
              Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
              eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut
              enim ad minim veniam, quis nostrud exercitation ullamco laboris.
            </p>
            <div className='flex gap-2'>
              <span className='rounded bg-blue-100 px-2 py-1 text-xs text-blue-800 dark:bg-blue-900 dark:text-blue-200'>
                Tag {i + 1}
              </span>
              <span className='rounded bg-green-100 px-2 py-1 text-xs text-green-800 dark:bg-green-900 dark:text-green-200'>
                Exemple
              </span>
            </div>
          </div>
        ))}
      </div>
    ),
    footer: (
      <div className='flex justify-between'>
        <Button variant='ghost'>Action secondaire</Button>
        <div className='flex gap-3'>
          <Button variant='ghost'>Annuler</Button>
          <Button variant='primary'>Sauvegarder</Button>
        </div>
      </div>
    )
  }
}

export const FormExample: Story = {
  render: args => {
    const [isOpen, setIsOpen] = useState(false)
    const [formData, setFormData] = useState({
      name: '',
      description: '',
      type: 'audit'
    })

    const handleSubmit = () => {
      setIsOpen(false)
    }

    return (
      <div className='flex min-h-screen items-center justify-center bg-gray-50 p-8 dark:bg-gray-900'>
        <Button onClick={() => setIsOpen(true)} variant='primary' size='lg'>
          Créer un projet
        </Button>

        <Modal
          {...args}
          isOpen={isOpen}
          onClose={() => setIsOpen(false)}
          title='Créer un nouveau projet'
          size='md'
          footer={
            <div className='flex justify-end gap-3'>
              <Button onClick={() => setIsOpen(false)} variant='ghost'>
                Annuler
              </Button>
              <Button
                onClick={handleSubmit}
                variant='primary'
                disabled={!formData.name.trim()}
              >
                Créer
              </Button>
            </div>
          }
        >
          <form className='space-y-6' onSubmit={e => e.preventDefault()}>
            <div>
              <label className='mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300'>
                Nom du projet *
              </label>
              <input
                type='text'
                value={formData.name}
                onChange={e =>
                  setFormData(prev => ({ ...prev, name: e.target.value }))
                }
                placeholder='Mon super projet'
                className='w-full rounded-md border border-gray-300 px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white'
                required
              />
            </div>

            <div>
              <label className='mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300'>
                Description
              </label>
              <textarea
                rows={4}
                value={formData.description}
                onChange={e =>
                  setFormData(prev => ({
                    ...prev,
                    description: e.target.value
                  }))
                }
                placeholder='Décrivez votre projet...'
                className='w-full rounded-md border border-gray-300 px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white'
              />
            </div>

            <div>
              <label className='mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300'>
                Type de projet
              </label>
              <select
                value={formData.type}
                onChange={e =>
                  setFormData(prev => ({ ...prev, type: e.target.value }))
                }
                className='w-full rounded-md border border-gray-300 px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-800 dark:text-white'
              >
                <option value='audit'>Audit de sécurité</option>
                <option value='analysis'>Analyse de code</option>
                <option value='pentest'>Test de pénétration</option>
                <option value='review'>Revue de code</option>
              </select>
            </div>

            <div className='rounded-lg bg-gray-50 p-4 dark:bg-gray-800'>
              <h5 className='mb-2 font-medium'>Aperçu</h5>
              <div className='space-y-1 text-sm text-gray-600 dark:text-gray-400'>
                <p>
                  <strong>Nom :</strong> {formData.name || 'Non défini'}
                </p>
                <p>
                  <strong>Type :</strong> {formData.type}
                </p>
                <p>
                  <strong>Description :</strong>{' '}
                  {formData.description || 'Aucune description'}
                </p>
              </div>
            </div>
          </form>
        </Modal>
      </div>
    )
  }
}
