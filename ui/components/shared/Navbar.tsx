/* eslint-disable @next/next/no-img-element */
"use client";

import React from "react";
import { usePathname, useRouter } from "next/navigation";
import { AiOutlineArrowLeft } from "react-icons/ai";
import { Button } from "../ui/button";

const Navbar = () => {
    const path = usePathname();
    const router = useRouter();

    const returnPageName = () => {
        if (path.includes("/") && (!path.includes("/user/") && !path.includes("/user"))) {
            return (
                <div className="flex items-center">
                    Overview
                </div>
            )
        }
        if (path.includes("/user") && (!path.includes("/user/"))) {
            return (
                <div className="flex items-center">
                    Users
                </div>
            )
        }
        if (path.includes("/user/")) {
            return (
                <div className="flex items-center gap-2">
                    <Button variant="ghost" onClick={() => router.back()}>
                        <AiOutlineArrowLeft size={25} />
                    </Button>
                    User Details
                </div>
            )
        }
    }

    return (
        <div className="flex justify-between pr-5 items-center h-20 border-b fixed w-screen top-0 z-50 bg-background">
            <div className="flex items-center h-28">
                <div className="p-3 w-56 border-r h-full items-center flex gap-2">
                    <img
                        src="/flexauth_logo.svg"
                        alt="flexauth-logo"
                        className="w-[40px] cursor-pointer"
                        onClick={() => router.push("/")}
                    />
                    <h1 className="text-2xl">FlexAuth</h1>
                </div>
                <h1 className="text-2xl font-medium text-accent-foreground ml-5">
                    {returnPageName()}
                </h1>
            </div>
        </div>
    );
};

export default Navbar;
