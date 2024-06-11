"use client"
import React from 'react'
import Users from '@/components/Users/Users'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useRouter } from 'next/navigation'

const UsersPage = () => {
    const router = useRouter();
    return (
        <div>
            {<Tabs defaultValue="users" className="w-full mb-4">
                <TabsList>
                    <TabsTrigger value="overview" onClick={() => router.push("/")}>Overview</TabsTrigger>
                    <TabsTrigger value="users" onClick={() => router.push("/user")}>Users</TabsTrigger>
                </TabsList>
            </Tabs>}
            <Users />
        </div>
    )
}

export default UsersPage