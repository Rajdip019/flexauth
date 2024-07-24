"use client"

import { Label, Pie, PieChart, Sector } from "recharts"

import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    ChartConfig,
    ChartContainer,
    ChartLegend,
    ChartLegendContent,
    ChartStyle,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart"
import React from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "../ui/select";
import { PieSectorDataItem } from "recharts/types/polar/Pie";


interface ChartPieProps {
    title: string;
    chartData: any;
    key: string;
    chartConfig: ChartConfig;
}

export function ChartPie({ title, chartData, chartConfig, key }: ChartPieProps) {
    const id = "pie-interactive"
    const [active, setActive] = React.useState(chartData[0].name)
    const activeIndex = React.useMemo(
        () => chartData.findIndex((item: any) => item.name === active),
        [active, chartData]
    )
    const options = React.useMemo(() => chartData.map((item: any) => item.name), [chartData])
    return (
        <Card data-chart={id} className="flex flex-col">
            <ChartStyle id={id} config={chartConfig} />
            <CardHeader className="flex-row items-start space-y-0 pb-0">
                <div className="grid gap-1">
                    <CardTitle className="text-xl">{title}</CardTitle>
                </div>
                <Select value={active} onValueChange={setActive}>
                    <SelectTrigger
                        className="ml-auto h-7 w-[130px] rounded-lg pl-2.5"
                        aria-label="Select a value"
                    >
                        <SelectValue placeholder="Select" />
                    </SelectTrigger>
                    <SelectContent align="end" className="rounded-xl">
                        {options.map((key: string) => {
                            const config = chartConfig[key as keyof typeof chartConfig]
                            if (!config) {
                                return null
                            }
                            return (
                                <SelectItem
                                    key={key}
                                    value={key}
                                    className="rounded-lg [&_span]:flex"
                                >
                                    <div className="flex items-center gap-2 text-xs">
                                        <span
                                            className="flex h-3 w-3 shrink-0 rounded-sm"
                                            style={{
                                                backgroundColor: `var(--color-${key})`,
                                            }}
                                        />
                                        {config?.label}
                                    </div>
                                </SelectItem>
                            )
                        })}
                    </SelectContent>
                </Select>
            </CardHeader>
            <CardContent className="flex flex-1 justify-center pb-0">
                <ChartContainer
                    id={id}
                    config={chartConfig}
                    className="mx-auto aspect-square w-full max-h-[250px]"
                >
                    <PieChart>
                        <ChartTooltip
                            cursor={false}
                            content={<ChartTooltipContent hideLabel />}
                        />
                        <Pie
                            data={chartData}
                            dataKey="count"
                            nameKey={key}
                            innerRadius={60}
                            strokeWidth={5}
                            activeIndex={activeIndex}
                            activeShape={({
                                outerRadius = 0,
                                ...props
                            }: PieSectorDataItem) => (
                                <g>
                                    <Sector {...props} outerRadius={outerRadius} />
                                    <Sector
                                        {...props}
                                        outerRadius={outerRadius + 10}
                                        innerRadius={outerRadius + 2}
                                    />
                                </g>
                            )}
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
                                                    {chartData[activeIndex].count.toLocaleString()}
                                                </tspan>
                                                <tspan
                                                    x={viewBox.cx}
                                                    y={(viewBox.cy || 0) + 24}
                                                    className="fill-muted-foreground"
                                                >
                                                    {chartData[activeIndex].name.toLocaleString()}
                                                </tspan>
                                            </text>
                                        )
                                    }
                                }}
                            />
                        </Pie>
                    </PieChart>
                </ChartContainer>
            </CardContent>
        </Card>
    )
}