import React from 'react'
import { IPages } from './Sidebar'
import Image from 'next/image'

interface Props {
    item: IPages
    path: string
    router: any
}

const SidebarItem: React.FC<Props> = ({ item, path, router }) => {
    const handleClickSidebarItems = (item: IPages) => {
        router.push(item.link)
    }

    const handleHighlight = (item: IPages) => {
        if (path.includes("/user/")) {
            return (item.link === "/user")
        }
        return (path === item.link);
    }

    return (
        <>
            {item.showOnSidebar && (
                <div key={item.name}>
                    <div className={`p-3 hover:bg-input mx-4 rounded cursor-pointer group ${handleHighlight(item) ? "bg-primary text-primary-foreground hover:bg-primary/90" : ""} my-1`} onClick={() => { handleClickSidebarItems(item) }}>
                        <div className='flex gap-4 items-center'>
                            {typeof item.icon === "string" ? <Image src={item.icon} alt="sidebar-icons" width={20} height={20} /> : item.icon}
                            <div className=' flex items-center justify-between w-full'>
                                <p className={`font-bold group-hover:opacity-100 transition-all text-lg`}>{item.name}</p>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </>
    )
}

export default SidebarItem