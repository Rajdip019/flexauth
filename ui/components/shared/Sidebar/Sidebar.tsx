"use client";

import React from 'react'
import { usePathname, useRouter } from 'next/navigation';
import SidebarItem from './SidebarItem';

export interface IPages {
    name: string;
    icon?: any;
    link: string;
    showOnSidebar?: boolean;
}

interface Props {
    items: IPages[]
}

const Sidebar: React.FC<Props> = ({ items }) => {
    const path = usePathname();
    const router = useRouter();

    return (
        <div className='w-56 h-[calc(100vh-4rem)] border-r pt-2 pb-5 flex flex-col justify-between fixed top-20'>
            <div>
                {items.map((item) => (
                    <SidebarItem key={item.name} item={item} path={path} router={router} />
                ))}
            </div>

            <div className='text-accent-foreground text-[11px] mt-4 flex justify-center'>
                <h1>© 2024 FlexAuth • All rights reserved</h1>
            </div>
        </div>
    )
}

export default Sidebar