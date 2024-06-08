"use client";
import { Loader } from '@/components/custom/Loader';
import { IUser } from '@/interfaces/IUser';
import React, { useEffect, useState } from 'react'
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from '@/components/ui/data-table';

const DashboardPage = () => {
    const [users, setUsers] = useState([] as IUser[])
    const [loading, setLoading] = useState(true)

    const getAllUsers = async () => {
        try{
            setLoading(true)
            const res = await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/user/get-all`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
            });
            const { data } = await res.json();
            setUsers(data);
        } catch (error) {
            console.error('Error during POST request:', error);
        }
        setLoading(false)
    }

    const columns: ColumnDef<IUser>[] = [
        {
            accessorKey: "name",
            header: "Name",
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
        },
        {
            accessorKey: "is_active",
            header: "Active",
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