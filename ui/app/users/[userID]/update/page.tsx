"use client";
import { Loader } from '@/components/custom/Loader';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import React, { useEffect, useState } from 'react'

const EditUser = ({ params }: any) => {
    const { userID } = params;
    const [email, setEmail] = useState('');
    const [name, setName] = useState('');
    const [loading, setLoading] = useState(false);

    const editUser = async () => {
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

    useEffect(() => {
        // fetch user by id
        const getUserByID = async () => {
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
                setEmail(data.email);
                setName(data.name);
            } catch (error) {
                console.error('Error during POST request:', error);
            }
            setLoading(false)
        }
        getUserByID();
    }, [userID])

    return (
        <div className='p-6'>
            {
                loading ?
                    <div className='h-[100vh] flex justify-center items-center'>
                        <Loader />
                    </div>
                    :
                    <div>
                        <div className='space-y-3'>
                            <h1 className='text-3xl text-primary'>Update User</h1>
                            <h1>Name</h1>
                            <Input type="text" placeholder="Name" value={name} onChange={(e) => setName(e.target.value)} />
                            <h1>Email</h1>
                            <Input type="text" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} />
                            <Button onClick={async () => await editUser()} disabled={loading}>
                                {loading ? <Loader /> : "Save"}
                            </Button>
                        </div>
                    </div>
            }
        </div>
    )
}

export default EditUser