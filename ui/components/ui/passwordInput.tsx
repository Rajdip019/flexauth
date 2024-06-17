import React, { useState } from 'react'
import { Input } from '../ui/input'
import { AiFillEye, AiFillEyeInvisible } from 'react-icons/ai'
import { Button } from './button';

interface Props {
    password: string;
    setPassword: (password: string) => void;
    label: string;
    placeholder?: string;
}

const PasswordInput: React.FC<Props> = ({ password, setPassword, label, placeholder }) => {
    const [showPassword, setShowPassword] = useState<boolean>(false);

    return (
        <div className="relative">
            <h1 className='mb-1'>
                {label}
            </h1>
            <Input
                type={showPassword ? 'text' : 'password'}
                placeholder={placeholder ? placeholder : 'Enter password'}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
            />
            {
                !showPassword ? <Button
                    variant="ghost"
                    className="absolute inset-y-6 right-0 flex items-center px-2 focus:outline-none"
                    onClick={() => setShowPassword(!showPassword)}
                >
                    <AiFillEye className="h-5 w-5 text-gray-500" />
                </Button> : <Button
                    variant="ghost"
                    className="absolute inset-y-6 right-0 flex items-center px-2 focus:outline-none"
                    onClick={() => setShowPassword(!showPassword)}
                >
                    <AiFillEyeInvisible className="h-5 w-5 text-gray-500" />
                </Button>
            }
        </div>
    )
}

export default PasswordInput