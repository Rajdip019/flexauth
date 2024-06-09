"use client";
import { Loader } from '@/components/custom/Loader';
import { IUser } from '@/interfaces/IUser';
import React, { useEffect, useState } from 'react'
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from '@/components/ui/data-table';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu';
import { IoMdMore } from 'react-icons/io';
import { useRouter } from 'next/navigation';
import { AlertDialog, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@/components/ui/alert-dialog';
import { Button } from '@/components/ui/button';
import { LuArrowUpRight } from 'react-icons/lu';
import { Input } from '@/components/ui/input';

const DashboardPage = () => {
    const [users, setUsers] = useState([] as IUser[])
    const [loading, setLoading] = useState(true)
    const [isDeleteLoading, setIsDeleteLoading] = useState(false)
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
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/delete`, {
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

    const editUser = async (email: string, name: string) => {
        try {
            setLoading(true)
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
                    <div className="flex w-[20vw] hover:underline group cursor-pointer items-center" onClick={() => router.push(`/user/${user.uid}`)}>
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
            header: "Email Verified",
            cell: ({ row }) => {
                return (
                    <div>
                        {row.original.email_verified ? 'True' : 'False'}
                    </div>
                )
            },
        },
        {
            accessorKey: "is_active",
            header: "Active",
            cell: ({ row }) => {
                return (
                    <div>
                        {row.original.is_active ? 'True' : 'False'}
                    </div>
                )
            },
        },
        {
            accessorKey: "created_at",
            header: "Created At",
            cell: ({ row }) => {
                return (
                    <div>
                        {new Date(parseInt(row.original.created_at.$date.$numberLong)).toLocaleString()}
                    </div>
                )
            },
        },
        {
            accessorKey: "updated_at",
            header: "Action",
            cell: ({ row }) => {
                const user = row.original;
                // eslint-disable-next-line react-hooks/rules-of-hooks
                const [name, setName] = useState(user.name);
                return (
                    <div>
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <IoMdMore size={20} />
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuItem asChild className="hover:bg-accent hover:cursor-pointer">
                                    <AlertDialog>
                                        <AlertDialogTrigger>
                                            <Button>
                                                Update
                                            </Button>
                                        </AlertDialogTrigger>
                                        <AlertDialogContent>
                                            <AlertDialogHeader>
                                                <AlertDialogTitle className='mb-2'>Update User Name</AlertDialogTitle>
                                                <AlertDialogDescription className='space-y-2'>
                                                    <h1>Name</h1>
                                                    <Input type="text" placeholder="Enter Name" value={name} onChange={(e) => setName(e.target.value)} />
                                                </AlertDialogDescription>
                                            </AlertDialogHeader>
                                            <AlertDialogFooter>
                                                <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                <Button variant="destructive" onClick={async () => {
                                                    setLoading(true);
                                                    await editUser(user.email, name);
                                                    await getAllUsers();
                                                    setLoading(false);
                                                }}>{isDeleteLoading ? <Loader /> : <h1>Continue</h1>}</Button>
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
                                                    setIsDeleteLoading(true);
                                                    await deleteUser(user.email);
                                                    await getAllUsers();
                                                    setIsDeleteLoading(false);
                                                }}>{isDeleteLoading ? <Loader /> : <h1>Continue</h1>}</Button>
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
        <div className='p-6'>
            <div>
                {
                    loading ?
                        <div className='h-[100vh] flex justify-center items-center'>
                            <Loader />
                        </div>
                        : <div>
                            <h1 className='text-3xl text-primary mb-4'>Dashboard</h1>
                            <DataTable
                                data={users ? users : []}
                                columns={columns}
                            />
                        </div>
                }
            </div>
        </div>
    )
}

export default DashboardPage