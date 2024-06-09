"use client";
import { Loader } from '@/components/custom/Loader';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { IUser } from '@/interfaces/IUser';
import { AlertDialog, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from '@/components/ui/alert-dialog';
import React, { useEffect } from 'react'
import { Input } from '@/components/ui/input';
import { Switch } from '@/components/ui/switch';
import { Label } from '@/components/ui/label';
import { MdEdit } from 'react-icons/md';
import { useRouter } from 'next/navigation';
import { ISession } from '@/interfaces/ISession';
import { ColumnDef } from '@tanstack/react-table';
import { LuArrowUpRight } from 'react-icons/lu';
import { DataTable } from '@/components/ui/data-table';

const UserDetails = ({ params }: any) => {
    const { userID } = params;
    const [loading, setLoading] = React.useState(true);
    const [user, setUser] = React.useState<IUser | null>(null);
    const [role, setRole] = React.useState('');
    const [sessions, setSessions] = React.useState([] as ISession[]);
    const router = useRouter();

    //function to update role
    const updateRole = async () => {
        try {
            setLoading(true)
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
            await getUser()
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
        setRole(user?.role || '')
    }

    // function to update is_active
    const updateUserActive = async (is_active: boolean) => {
        try {
            setLoading(true)
            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/toggle-active-status`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    email: user?.email,
                    is_active: is_active
                }),
            });
            await getUser()
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

    const getUser = async () => {
        try {
            setLoading(true)
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/get-from-uid`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    uid: userID
                }),
            });
            const { data } = await res.json();
            setUser(data);
            setRole(data?.role || '')
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

    // fetch all sessions
    const fetchAllSessions = async () => {
        try {
            setLoading(true)
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/get-all-from-uid`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    uid: userID
                }),
                cache: 'no-cache',
            });
            const { data } = await res.json();
            setSessions(data);
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
            router.push('/');
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }


    const sessionColumns: ColumnDef<ISession>[] = [
        {
            accessorKey: "uid",
            header: "UID",
            cell: ({ row }) => {
                const session = row.original;
                return (
                    <div
                        className="flex w-[20vw] hover:underline group cursor-pointer items-center"
                        onClick={() => router.push(`/sessions/${session.uid}`)}
                    >
                        <div>{session.uid}</div>
                        <div className="ml-1 underline hidden group-hover:block transition-all duration-300 ease-in-out">
                            <LuArrowUpRight className="ml-1" size={16} />
                        </div>
                    </div>
                );
            },
        },
        {
            accessorKey: "email",
            header: "Email",
        },
        {
            accessorKey: "user_agent",
            header: "User Agent",
        },
        {
            accessorKey: "is_revoked",
            header: "Revoked",
            cell: ({ row }) => {
                return <div>{row.original.is_revoked ? 'True' : 'False'}</div>;
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
                );
            },
        },
        {
            accessorKey: "updated_at",
            header: "Updated At",
            cell: ({ row }) => {
                return (
                    <div>
                        {new Date(parseInt(row.original.updated_at.$date.$numberLong)).toLocaleString()}
                    </div>
                );
            },
        },
    ];

    useEffect(() => {
        getUser()
        fetchAllSessions()
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
                            <div className='flex justify-between items-center'>
                                <h1 className='text-3xl text-primary mb-4'>User Details</h1>
                                <div className='space-x-2'>
                                    <Button onClick={() => router.push(`/users/${userID}/update`)}>Update</Button>
                                    <AlertDialog>
                                        <AlertDialogTrigger>
                                            <Button variant="destructive">
                                                Delete
                                            </Button>
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
                                                    await deleteUser(user?.email!);
                                                }}>{loading ? <Loader /> : <h1>Continue</h1>}</Button>
                                            </AlertDialogFooter>
                                        </AlertDialogContent>
                                    </AlertDialog>
                                </div>
                            </div>
                            <Card className="w-full">
                                <CardHeader>
                                    <CardTitle>{user?.name}</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className='grid grid-cols-3 gap-5'>
                                        <div>
                                            <p className='text-sm text-gray-500'>Email</p>
                                            <p className='text-lg'>{user?.email}</p>
                                        </div>
                                        <div>
                                            <p className='text-sm text-gray-500'>Role</p>
                                            <div className='flex gap-2 items-center'>
                                                <p className='text-lg'>{user?.role}</p>
                                                <AlertDialog>
                                                    <AlertDialogTrigger>
                                                        <Button variant="ghost" className='p-2'>
                                                            <MdEdit />
                                                        </Button>
                                                    </AlertDialogTrigger>
                                                    <AlertDialogContent>
                                                        <AlertDialogHeader>
                                                            <AlertDialogTitle className='mb-2'>Change User Role</AlertDialogTitle>
                                                            <AlertDialogDescription>
                                                                <Input type="text" placeholder="Enter Role" value={role} onChange={(e) => setRole(e.target.value)} />
                                                            </AlertDialogDescription>
                                                        </AlertDialogHeader>
                                                        <AlertDialogFooter>
                                                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                            <Button variant="destructive" onClick={async () => {
                                                                setLoading(true);
                                                                await updateRole();
                                                                setLoading(false);
                                                            }}>{loading ? <Loader /> : <h1>Continue</h1>}</Button>
                                                        </AlertDialogFooter>
                                                    </AlertDialogContent>
                                                </AlertDialog>
                                            </div>
                                        </div>
                                        <div>
                                            <p className='text-sm text-gray-500'>Email Verified</p>
                                            <p className='text-lg'>{user?.email_verified.toString()}</p>
                                        </div>
                                        <div>
                                            <p className='text-sm text-gray-500'>Is Active</p>
                                            <div className="flex items-center space-x-2">
                                                <Switch id="is_active" checked={user?.is_active} onCheckedChange={async (val) => await updateUserActive(val)} />
                                                <Label htmlFor="is_active">{loading ? <Loader /> : user?.is_active.toString()}</Label>
                                            </div>
                                        </div>
                                        <div>
                                            <p className='text-sm text-gray-500'>Created At</p>
                                            <p className='text-lg'>
                                                {new Date(parseInt(user?.created_at.$date.$numberLong!)).toLocaleString()}
                                            </p>
                                        </div>
                                        <div>
                                            <p className='text-sm text-gray-500'>Updated At</p>
                                            <p className='text-lg'>
                                                {new Date(parseInt(user?.updated_at.$date.$numberLong!)).toLocaleString()}
                                            </p>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                            <div>
                                <h1 className='text-2xl text-primary my-6'>All Sessions</h1>
                                <DataTable
                                    data={sessions ? sessions : []}
                                    columns={sessionColumns}
                                />
                            </div>
                        </div>
                }
            </div>
        </div>
    )
}

export default UserDetails