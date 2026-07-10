import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'Sustaina Dependency Explorer',
  description: 'Visualize and fund open-source dependencies on Stellar',
  viewport: 'width=device-width, initial-scale=1',
  authors: [{ name: 'Drips Network' }],
  keywords: ['Sustaina', 'Stellar', 'Soroban', 'Dependencies', 'Funding', 'Open Source'],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>{children}</body>
    </html>
  );
}
