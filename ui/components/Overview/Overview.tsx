/* eslint-disable react-hooks/rules-of-hooks */
"use client";
import React, { useEffect, useState } from 'react'
import { Loader } from '../custom/Loader';
import { IOverview } from '@/interfaces/IOverview';
import { DonutChartStats } from '../custom/DonutChartForStats';
import { ChartConfig } from '../ui/chart';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { AlertDialog, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@/components/ui/alert-dialog';
import { FaUsersSlash } from 'react-icons/fa';
import { ChartPie } from '../custom/PieChart';
import { IUser } from '@/interfaces/IUser';
import { ColumnDef } from '@tanstack/react-table';
import { LuArrowUpRight } from 'react-icons/lu';
import { useRouter } from 'next/navigation';
import { Badge } from '../ui/badge';
import { TiTick } from 'react-icons/ti';
import { GoClockFill } from 'react-icons/go';
import { IoIosWarning, IoMdMore } from 'react-icons/io';
import { format } from 'date-fns';
import { toast } from '../ui/use-toast';
import { Input } from '../ui/input';
import { Button } from '../ui/button';
import { DataTable } from '../ui/data-table';

const Overview = () => {
    const [overview, setOverview] = useState<IOverview | null>(null)
    const [recentUsers, setRecentUsers] = useState<IUser[]>([])
    const [loading, setLoading] = useState(true)

    const userChartData = [
        { name: "active", count: overview?.active_user_count, fill: "var(--color-active)" },
        { name: "inactive", count: overview?.inactive_user_count, fill: "var(--color-inactive)" },
    ]

    const userChartConfig = {
        user: {
            label: "Users",
        },
        active: {
            label: "Active",
            color: "hsl(var(--chart-1-1))",
        },
        inactive: {
            label: "Inactive",
            color: "hsl(var(--chart-2-1))",
        },
    } satisfies ChartConfig

    const sessionChartData = [
        { name: "active", count: overview?.active_session_count, fill: "var(--color-active)" },
        { name: "revoked", count: overview?.revoked_session_count, fill: "var(--color-revoked)" },
    ]

    const sessionChartConfig = {
        session: {
            label: "Sessions",
        },
        active: {
            label: "Active",
            color: "hsl(var(--chart-1-1))",
        },
        revoked: {
            label: "Revoked",
            color: "hsl(var(--chart-2-2))",
        },
    } satisfies ChartConfig


    // Define an interface for the device counts
    interface CountObject {
        [key: string]: number;
    }

    // Initialize the device counts object with the correct type
    const deviceCounts: CountObject = (overview ?? {
        device_types: [],
    }).device_types.reduce((acc, device) => {
        const sanitizedKey = device.replace(/\s+/g, ''); // Remove whitespace from the key
        acc[sanitizedKey] = (acc[sanitizedKey] || 0) + 1;
        return acc;
    }, {} as CountObject);

    // Count occurrences in the browsers array
    const browserCounts: CountObject = (overview ?? {
        browser_types: [],
    }).browser_types.reduce((acc, browser) => {
        const sanitizedKey = browser.replace(/\s+/g, ''); // Remove whitespace from the key
        acc[sanitizedKey] = (acc[sanitizedKey] || 0) + 1;
        return acc;
    }, {} as CountObject);

    // Count occurrences in the OS types array
    const osTypeCounts: CountObject = (overview ?? {
        os_types: [],
    }).os_types.reduce((acc, os) => {
        const sanitizedKey = os.replace(/\s+/g, ''); // Remove whitespace from the key
        acc[sanitizedKey] = (acc[sanitizedKey] || 0) + 1;
        return acc;
    }, {} as CountObject);


    // Define a function to generate colors dynamically
    const generateColor = (index: number, themeNo: number) => {
        return `hsl(var(--chart-${index + 1}-${themeNo}))`;
    };

    // Generic function to generate chart configuration
    const generateChartConfig = (counts: CountObject, themeNo: number) => {
        const config: { [key: string]: { label: string; color: string } } = {};
        const types = Object.keys(counts);


        types.forEach((type, index) => {
            config[type] = {
                label: type,
                color: generateColor(index, themeNo),
            };
        });

        return config;
    };

    const sessionDeviceChartData = Object.keys(deviceCounts).map(device => ({
        name: device,
        count: deviceCounts[device] || 0,
        fill: `var(--color-${device})`
    }));

    const sessionBrowserChartData = Object.keys(browserCounts).map(browser => ({
        name: browser,
        count: browserCounts[browser] || 0,
        fill: `var(--color-${browser})`
    }));

    const sessionOsTypeChartData = Object.keys(osTypeCounts).map(os => ({
        name: os,
        count: osTypeCounts[os] || 0,
        fill: `var(--color-${os})`
    }));

    const sessionDeviceChartConfig = generateChartConfig(deviceCounts, 2);

    const sessionBrowserChartConfig = generateChartConfig(browserCounts, 1);

    const sessionOSChartConfig = generateChartConfig(osTypeCounts, 2);

    // delete user function
    const deleteUser = async (email: string) => {
        try {
            setLoading(true)
            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/delete`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    email
                }),
            });
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

    const router = useRouter();

    const columns: ColumnDef<IUser>[] = [
        {
            accessorKey: "name",
            header: "Name",
            cell: ({ row }) => {
                const user = row.original;
                return (
                    <div className="flex w-44 hover:underline truncate group cursor-pointer items-center" onClick={() => router.push(`/users/${user.uid}`)}>
                        <div>{user.name}</div>
                        <div
                            className="ml-1 underline hidden group-hover:block transition-all duration-300 ease-in-out"
                        >
                            <LuArrowUpRight className="ml-1" size={16} />
                        </div>
                    </div>
                );
            }
        },
        {
            accessorKey: "email",
            header: "Email",
        },
        {
            accessorKey: "role",
            header: "Role",
        },
        {
            accessorKey: "email_verified",
            header: "Email Verification",
            cell: ({ row }) => {
                return (
                    <Badge
                        className={`${row.original?.email_verified ? "bg-green-500 hover:bg-green-500" : "bg-yellow-500 hover:bg-yellow-500"
                            } flex gap-1 w-fit rounded`}
                    >
                        {row.original?.email_verified ? <TiTick /> : <GoClockFill />}

                        {row.original?.email_verified ? "Verified" : "Pending"}
                    </Badge>
                )
            },
        },
        {
            accessorKey: "is_active",
            header: "Account Status",
            cell: ({ row }) => {
                return (
                    <Badge
                        className={`${row.original?.is_active
                            ? "bg-green-500 hover:bg-green-500"
                            : "bg-red-500 text-white hover:bg-red-500"
                            } flex gap-1 w-fit rounded`}
                    >
                        {row.original?.is_active ? <TiTick /> : <IoIosWarning />}

                        {row.original?.is_active ? "Active" : "Suspended"}
                    </Badge>
                )
            },
        },
        {
            accessorKey: "created_at",
            header: "Created At",
            cell: ({ row }) => {
                return (
                    <div className='w-fit'>
                        {format(
                            new Date(parseInt(row.original?.created_at.$date.$numberLong!)),
                            "PP - p"
                        )}
                    </div>
                )
            },
        },
        {
            accessorKey: "updated_at",
            header: "Action",
            cell: ({ row }) => {
                const user = row.original;
                const [name, setName] = useState(user.name);
                const [role, setRole] = useState(user.role)
                // edit user function
                const editUser = async (email: string, name: string, role: string) => {
                    if (name === "" || role === "") {
                        toast({
                            title: "Error",
                            description: "Fill all the fields correctly.",
                            variant: "destructive"
                        });
                        return;
                    };
                    try {
                        setLoading(true)
                        if (role !== user?.role) {
                            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/update-role`, {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json',
                                },
                                body: JSON.stringify({
                                    email: user?.email,
                                    role
                                }),
                            });
                        }
                        if (name !== user?.name) {
                            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/update`, {
                                method: 'POST',
                                headers: {
                                    'Content-Type': 'application/json',
                                },
                                body: JSON.stringify({
                                    email,
                                    name
                                }),
                            });
                        }
                        await getRecentUsers()
                    } catch (error) {
                        console.error('Error during POST request:', error);
                    }
                    setLoading(false)
                }

                return (
                    <div>
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <IoMdMore size={20} />
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem asChild className="hover:bg-accent hover:cursor-pointer">
                                    <AlertDialog>
                                        <AlertDialogTrigger className="relative flex items-center w-32 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent cursor-pointer">
                                            Update
                                        </AlertDialogTrigger>
                                        <AlertDialogContent>
                                            <AlertDialogHeader>
                                                <AlertDialogTitle className='mb-2'>Update User Name</AlertDialogTitle>
                                                <AlertDialogDescription className='space-y-2'>
                                                    <h1>Name</h1>
                                                    <Input type="text" placeholder="Enter Name" value={name} onChange={(e) => setName(e.target.value)} />
                                                    <h1>Role</h1>
                                                    <Input type="text" placeholder="Enter Role" value={role} onChange={(e) => setRole(e.target.value)} />
                                                </AlertDialogDescription>
                                            </AlertDialogHeader>
                                            <AlertDialogFooter>
                                                <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                <Button variant="destructive" onClick={async () => {
                                                    await editUser(user.email, name, role);
                                                }}>{loading ? <Loader /> : <h1>Continue</h1>}</Button>
                                            </AlertDialogFooter>
                                        </AlertDialogContent>
                                    </AlertDialog>
                                </DropdownMenuItem>
                                <DropdownMenuItem asChild className="hover:bg-accent hover:cursor-pointer">
                                    <AlertDialog>
                                        <AlertDialogTrigger className="relative flex items-center w-32 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent cursor-pointer">
                                            Delete
                                        </AlertDialogTrigger>
                                        <AlertDialogContent>
                                            <AlertDialogHeader>
                                                <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                                                <AlertDialogDescription>
                                                    This action cannot be undone.
                                                </AlertDialogDescription>
                                            </AlertDialogHeader>
                                            <AlertDialogFooter>
                                                <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                <Button variant="destructive" onClick={async () => {
                                                    setLoading(true);
                                                    await deleteUser(user.email);
                                                    await getRecentUsers();
                                                    setLoading(false);
                                                }}>{loading ? <Loader /> : <h1>Continue</h1>}</Button>
                                            </AlertDialogFooter>
                                        </AlertDialogContent>
                                    </AlertDialog>
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                )
            },
        },
    ];


    const getOverview = async () => {
        try {
            setLoading(true)
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/overview/get-all`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
                cache: 'no-cache',
            });
            const { data } = await res.json();
            setOverview(data);
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

    const getRecentUsers = async () => {
        try {
            setLoading(true)
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/get-recent`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ limit: 5 }),
                cache: 'no-cache',
            });
            const { data } = await res.json();
            setRecentUsers(data);
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

    const fetchAllData = async () => {
        await Promise.all([getOverview(), getRecentUsers()])
    }

    useEffect(() => {
        fetchAllData()
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [])

    return (
        <div>
            {
                loading ?
                    <div className='h-[calc(100vh-10rem)] flex justify-center items-center'>
                        <Loader />
                    </div>
                    :
                    <div>
                        <div className='grid grid-cols-3 gap-5'>
                            <DonutChartStats
                                title='Total Users'
                                chartData={userChartData}
                                chartConfig={userChartConfig}
                                key='name'
                            />
                            <Card className='flex flex-col'>
                                <CardHeader>
                                    <CardTitle className="text-xl mb-10">Blocked Users</CardTitle>
                                </CardHeader>
                                <CardContent className='flex gap-10 items-end justify-center'>
                                    <FaUsersSlash size={120} className='text-gray-300' />
                                    <p className="text-5xl font-bold mb-4">
                                        {overview?.blocked_user_count}
                                    </p>
                                </CardContent>
                            </Card>
                            <DonutChartStats
                                title='Total Sessions'
                                chartData={sessionChartData}
                                chartConfig={sessionChartConfig}
                                key='name'
                            />
                            <ChartPie
                                title='All Devices'
                                chartData={sessionDeviceChartData}
                                chartConfig={sessionDeviceChartConfig}
                                key='name'
                            />
                            <ChartPie
                                title='All Browsers'
                                chartData={sessionBrowserChartData}
                                chartConfig={sessionBrowserChartConfig}
                                key='name'
                            />
                            <ChartPie
                                title='Operating Systems'
                                chartData={sessionOsTypeChartData}
                                chartConfig={sessionOSChartConfig}
                                key='name'
                            />
                        </div>
                        <div className='mt-5'>
                            <h1 className='font-bold text-xl my-3'>Recent Users</h1>
                            <DataTable
                                data={recentUsers ? recentUsers : []}
                                columns={columns}
                            />
                        </div>
                    </div>
            }
        </div>
    )
}

export default Overview