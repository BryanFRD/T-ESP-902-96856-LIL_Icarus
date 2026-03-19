import type { Meta, StoryObj } from '@storybook/react'
import { Building2, Calendar, MapPin } from 'lucide-react'

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from '../components/oldUi/card/Card'

const meta: Meta<typeof Card> = {
  title: 'UI/Card',
  component: Card,
  parameters: {
    layout: 'centered'
  },
  tags: ['autodocs']
}

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {},
  render: () => (
    <Card>
      <CardHeader>
        <CardTitle>Titre de la carte</CardTitle>
        <CardDescription>Description de la carte</CardDescription>
      </CardHeader>
      <CardContent>
        <p className='text-gray-700 dark:text-gray-300'>
          Contenu de la carte avec du texte exemple.
        </p>
      </CardContent>
    </Card>
  )
}

export const WithHover: Story = {
  args: {},
  render: () => (
    <Card hover>
      <CardHeader>
        <CardTitle>Carte interactive</CardTitle>
        <CardDescription>Passez la souris dessus</CardDescription>
      </CardHeader>
      <CardContent>
        <p className='text-gray-700 dark:text-gray-300'>
          Cette carte a un effet au survol.
        </p>
      </CardContent>
    </Card>
  )
}

export const WithIcon: Story = {
  args: {},
  render: () => (
    <Card hover className='w-80'>
      <CardHeader>
        <div className='mb-3 flex items-start justify-between'>
          <div className='flex h-10 w-10 items-center justify-center rounded-lg bg-orange-50 text-orange-600 dark:bg-orange-900/20 dark:text-orange-400'>
            <Building2 size={20} />
          </div>
          <span className='inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800 dark:bg-green-900/30 dark:text-green-300'>
            Actif
          </span>
        </div>
        <CardTitle>Projet Icarus</CardTitle>
        <CardDescription>Application web moderne</CardDescription>
      </CardHeader>
      <CardContent>
        <div className='space-y-2'>
          <div className='flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400'>
            <Calendar size={14} />
            <span>Créé le 6 février 2026</span>
          </div>
          <div className='flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400'>
            <MapPin size={14} />
            <span>Paris, France</span>
          </div>
        </div>
      </CardContent>
      <CardFooter>
        <button className='text-sm font-medium text-orange-600 hover:text-orange-700 dark:text-orange-400 dark:hover:text-orange-300'>
          Voir les détails →
        </button>
      </CardFooter>
    </Card>
  )
}

export const Outlined: Story = {
  args: {},
  render: () => (
    <Card variant='outlined' className='w-80'>
      <CardHeader>
        <CardTitle>Carte outlined</CardTitle>
        <CardDescription>Variante avec bordure uniquement</CardDescription>
      </CardHeader>
      <CardContent>
        <p className='text-gray-700 dark:text-gray-300'>
          Cette carte utilise la variante &ldquo;outlined&rdquo;.
        </p>
      </CardContent>
    </Card>
  )
}

export const Elevated: Story = {
  args: {},
  render: () => (
    <Card variant='elevated' className='w-80'>
      <CardHeader>
        <CardTitle>Carte élevée</CardTitle>
        <CardDescription>Avec une ombre</CardDescription>
      </CardHeader>
      <CardContent>
        <p className='text-gray-700 dark:text-gray-300'>
          Cette carte utilise la variante &ldquo;elevated&rdquo; avec une ombre.
        </p>
      </CardContent>
    </Card>
  )
}

export const WithAnimation: Story = {
  args: {},
  render: () => (
    <div className='flex gap-4'>
      <Card animate animationDelay={0} className='w-60'>
        <CardHeader>
          <CardTitle>Carte 1</CardTitle>
          <CardDescription>Animation instantanée</CardDescription>
        </CardHeader>
      </Card>
      <Card animate animationDelay={0.1} className='w-60'>
        <CardHeader>
          <CardTitle>Carte 2</CardTitle>
          <CardDescription>Délai de 0.1s</CardDescription>
        </CardHeader>
      </Card>
      <Card animate animationDelay={0.2} className='w-60'>
        <CardHeader>
          <CardTitle>Carte 3</CardTitle>
          <CardDescription>Délai de 0.2s</CardDescription>
        </CardHeader>
      </Card>
    </div>
  )
}

export const NoPadding: Story = {
  args: {},
  render: () => (
    <Card padding='none' className='w-80 overflow-hidden'>
      <div className='h-32 bg-linear-to-r from-red-500 to-orange-500' />
      <div className='p-5'>
        <CardTitle className='mb-2'>Image en haut</CardTitle>
        <CardDescription>
          Carte sans padding par défaut pour un contrôle total
        </CardDescription>
      </div>
    </Card>
  )
}

export const WithClickHandler: Story = {
  args: {},
  render: () => (
    <Card onClick={() => alert('Carte cliquée!')} hover className='w-80'>
      <CardHeader>
        <CardTitle>Carte cliquable</CardTitle>
        <CardDescription>
          Cliquez n&apos;importe où sur cette carte
        </CardDescription>
      </CardHeader>
      <CardContent>
        <p className='text-gray-700 dark:text-gray-300'>
          Cette carte a un gestionnaire onClick.
        </p>
      </CardContent>
    </Card>
  )
}
