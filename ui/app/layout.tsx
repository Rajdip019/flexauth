import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import "./globals.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import Navbar from "@/components/shared/Navbar";
import { Toaster } from "@/components/ui/toaster";
import Sidebar from "@/components/shared/Sidebar/Sidebar";
import { AppPages } from "@/constants/appconstants";

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
              <div className="flex items-start min-h-[calc(100vh-4rem)]">
                <Sidebar items={AppPages} />
                <div className="p-4 ml-56 min-h-[calc(100vh-5rem)] w-[calc(100vw-14rem)] mt-20">
                  {children}
                </div>
              </div>
            </main>
            <Toaster />
          </ TooltipProvider>
        </div>
      </body>
    </html>
  );
}
