import { IPages } from "@/components/shared/Sidebar/Sidebar";
import { FaUsers } from "react-icons/fa";
import { IoStatsChartSharp } from "react-icons/io5";

export const AppPages: IPages[] = [
    {
        name: 'Overview',
        icon: <IoStatsChartSharp size={24} />,
        link: '/',
        showOnSidebar: true,
    },
    {
        name: "Users",
        icon: <FaUsers size={24} />,
        link: '/user',
        showOnSidebar: true,
    },
]