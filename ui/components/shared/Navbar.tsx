/* eslint-disable @next/next/no-img-element */
"use client";

import React from "react";
import { usePathname, useRouter } from "next/navigation";
import { AiOutlineArrowLeft } from "react-icons/ai";
import { Button } from "../ui/button";

const Navbar = () => {
    const path = usePathname();
    const router = useRouter();

    // get the current page name from the path and AppPages also try to find if there is any subItem matches the path and return the SubItem name instead of the main page name
    // const currentPage = AppPages.find((item) => {
    //     if (item.link[0] === path) {
    //         return true;
    //     }
    //     return false;
    // });

    const returnPageName = () => {
        if (path.includes("/") && (!path.includes("/user/"))) {
            return (
                <div className="flex items-center">
                    Dashboard
                </div>
            )
        }
        if (path.includes("/user/")) {
            return (
                <div className=" flex items-center">
                    <Button variant="ghost" onClick={() => router.back()}>
                        <AiOutlineArrowLeft size={25} />
                    </Button>
                    User Details
                </div>
            )
        }
    }

    return (
        <div className="flex justify-between p-6 px-12 items-center h-16 border-b fixed w-screen top-0 z-50">
            <div className="flex items-center">
                <img
                    src="/flexauth_logo.svg"
                    alt="flexauth-logo"
                    className="w-[50px] cursor-pointer"
                    onClick={() => router.push("/")}
                />
                <h1 className="text-2xl font-medium text-accent-foreground ml-5">
                    {returnPageName()}
                </h1>
            </div>
        </div>
    );
};

export default Navbar;
