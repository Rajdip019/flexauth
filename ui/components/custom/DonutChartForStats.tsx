"use client"

import * as React from "react"
import { Label, Pie, PieChart } from "recharts"

import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart"
import { FaClock, FaUsersSlash } from "react-icons/fa"

interface DonutChartProps {
    title: string;
    chartData: any;
    key: string;
    chartConfig: ChartConfig;
}

export function DonutChartStats({ title, chartData, chartConfig, key }: DonutChartProps) {
    const totalCount = React.useMemo(() => {
        return chartData.reduce((acc: any, curr: { count: any }) => acc + curr.count, 0)
    }, [chartData])

    return (
        <Card className="flex flex-col">
            <CardHeader className="items-start pb-0">
                <CardTitle className="text-xl">{title}</CardTitle>
            </CardHeader>
            <CardContent className="flex-1 pb-0">
                <ChartContainer
                    config={chartConfig}
                    className="mx-auto aspect-square max-h-[250px]"
                >
                    {totalCount === 0 ?
                        <CardContent className='flex gap-10 items-end justify-center mt-16'>
                            <FaClock size={120} className='text-gray-300' />
                            <p className="text-5xl font-bold mb-4">
                                {totalCount}
                            </p>
                        </CardContent>
                        : <PieChart>
                            <ChartTooltip
                                cursor={false}
                                content={<ChartTooltipContent />}
                            />
                            <Pie
                                data={chartData}
                                dataKey="count"
                                nameKey={key}
                                innerRadius={60}
                                strokeWidth={5}
                            >
                                <Label
                                    content={({ viewBox }) => {
                                        if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                                            return (
                                                <text
                                                    x={viewBox.cx}
                                                    y={viewBox.cy}
                                                    textAnchor="middle"
                                                    dominantBaseline="middle"
                                                >
                                                    <tspan
                                                        x={viewBox.cx}
                                                        y={viewBox.cy}
                                                        className="fill-foreground text-3xl font-bold"
                                                    >
                                                        {totalCount.toLocaleString()}
                                                    </tspan>
                                                    <tspan
                                                        x={viewBox.cx}
                                                        y={(viewBox.cy || 0) + 24}
                                                        className="fill-muted-foreground"
                                                    >
                                                        {title}
                                                    </tspan>
                                                </text>
                                            )
                                        }
                                    }}
                                />
                            </Pie>
                        </PieChart>}
                </ChartContainer>
            </CardContent>
        </Card>
    )
}
