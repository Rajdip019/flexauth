"use client";
import React, { useEffect, useState } from 'react'
import { Loader } from '../custom/Loader';
import { IOverview } from '@/interfaces/IOverview';
import { DonutChartStats } from '../custom/DonutChartForStats';
import { ChartConfig } from '../ui/chart';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { FaUsersSlash } from 'react-icons/fa';
import { ChartPie } from '../custom/PieChart';

const Overview = () => {
    const [overview, setOverview] = useState<IOverview | null>(null)
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
        acc[device] = (acc[device] || 0) + 1;
        return acc;
    }, {} as CountObject);

    // Count occurrences in the browsers array
    const browserCounts: CountObject = (overview ?? {
        browser_types: [],
    }).browser_types.reduce((acc, browser) => {
        acc[browser] = (acc[browser] || 0) + 1;
        return acc;
    }, {} as CountObject);

    // Count occurrences in the OS types array
    const osTypeCounts: CountObject = (overview ?? {
        os_types: [],
    }).os_types.reduce((acc, os) => {
        acc[os] = (acc[os] || 0) + 1;
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

    useEffect(() => {
        getOverview()
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
                    </div>
            }
        </div>
    )
}

export default Overview