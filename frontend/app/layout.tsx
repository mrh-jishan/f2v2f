import '../styles/globals.css'

export const metadata = {
  title: 'f2v2f - File to Video Converter',
  description: 'Convert files to beautiful artistic videos and back',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 text-white">
        <div className="min-h-screen">
          {children}
        </div>
      </body>
    </html>
  )
}
