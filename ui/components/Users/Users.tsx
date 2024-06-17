/* eslint-disable react-hooks/rules-of-hooks */
"use client";

import { IUser } from '@/interfaces/IUser';
import { ColumnDef } from '@tanstack/react-table';
import { useRouter } from 'next/navigation';
import React, { useEffect, useState } from 'react'
import { LuArrowUpRight } from 'react-icons/lu';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { AlertDialog, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@/components/ui/alert-dialog';
import { Button } from '@/components/ui/button';
import { IoIosWarning, IoMdMore } from 'react-icons/io';
import { Input } from '@/components/ui/input';
import { Loader } from '@/components/custom/Loader';
import { DataTable } from '@/components/ui/data-table';
import { toast } from '@/components/ui/use-toast';
import { Badge } from '../ui/badge';
import { TiTick } from 'react-icons/ti';
import { GoClockFill } from 'react-icons/go';
import { format } from 'date-fns';


const Users = () => {
    const [users, setUsers] = useState([] as IUser[])
    const [loading, setLoading] = useState(true)
    const router = useRouter();

    const getAllUsers = async () => {
        try {
            setLoading(true)
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/get-all`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
                cache: 'no-cache',
            });
            const { data } = await res.json();
            setUsers(data);
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

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


    const columns: ColumnDef<IUser>[] = [
        {
            accessorKey: "name",
            header: "Name",
            cell: ({ row }) => {
                const user = row.original;
                return (
                    <div className="flex w-44 hover:underline truncate group cursor-pointer items-center" onClick={() => router.push(`/user/${user.uid}`)}>
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
                        await getAllUsers()
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
                                                    await getAllUsers();
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

    useEffect(() => {
        getAllUsers()
    }, [])
    return (
        <div>
            {
                loading ?
                    <div className='h-[calc(100vh-10rem)] flex justify-center items-center'>
                        <Loader />
                    </div>
                    : <div>
                        <DataTable
                            data={users ? users : []}
                            columns={columns}
                        />
                    </div>
            }
        </div>
    )
}

export default Users