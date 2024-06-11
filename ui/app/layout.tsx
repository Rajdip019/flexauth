import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import "./globals.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import Navbar from "@/components/shared/Navbar";
import { Toaster } from "@/components/ui/toaster";

const poppins = Poppins({
  weight: '400',
  subsets: ['latin'],
  display: 'swap',
})

export const metadata: Metadata = {
  title: "FlexAuth Dashboard",
  description: "Your own flexible, blazingly fast 🦀, and secure in-house authentication system.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={poppins.className}>
        <div className="hidden sm:block">
          <TooltipProvider>
            <main className="w-screen">
              <Navbar />
              <div className="h-[calc(100vh-4rem)] mt-12 p-6">
                {children}
              </div>
            </main>
            <Toaster />
          </ TooltipProvider>
        </div>
      </body>
    </html>
  );
}
