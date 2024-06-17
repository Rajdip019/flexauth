/* eslint-disable @next/next/no-img-element */
import { ISession } from "@/interfaces/ISession";
import {
    AlertDialog,
    AlertDialogTrigger,
    AlertDialogContent,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogCancel,
} from "@/components/ui/alert-dialog";
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuContent,
    DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { ColumnDef } from "@tanstack/react-table";
import { Loader } from "lucide-react";
import React, { useCallback, useEffect } from "react";
import { IoMdMore } from "react-icons/io";
import { AlertDialogHeader, AlertDialogFooter } from "../ui/alert-dialog";
import { Button } from "../ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "../ui/card";
import { DataTable } from "../ui/data-table";
import UAParser from "ua-parser-js";
import { addDays, format } from "date-fns";

interface SessionTableProps {
    userID: string;
}

const SessionTable: React.FC<SessionTableProps> = ({ userID }) => {
    const [loading, setLoading] = React.useState(false);
    const [sessions, setSessions] = React.useState([]);

    // fetch all sessions
    const fetchAllSessions = useCallback(async () => {
        try {
            setLoading(true);
            const res = await fetch(
                `${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/get-all-from-uid`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        uid: userID,
                    }),
                    cache: "no-cache",
                }
            );
            const { data } = await res.json();
            setSessions(data);
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    }, [userID]);

    // revoke session function
    const revokeSession = async (session_id: string) => {
        try {
            setLoading(true);
            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/revoke`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    session_id,
                    uid: userID,
                }),
            });
            await fetchAllSessions();
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    };

    // delete session function
    const deleteSession = async (session_id: string) => {
        try {
            setLoading(true);
            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/delete`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    session_id,
                    uid: userID,
                }),
            });
            await fetchAllSessions();
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    };

    // delete all sessions function
    const deleteAllSessions = async () => {
        try {
            setLoading(true);
            await fetch(
                `${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/delete-all`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        uid: userID,
                    }),
                }
            );
            await fetchAllSessions();
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    };

    // revoke all sessions function
    const revokeAllSessions = async () => {
        try {
            setLoading(true);
            await fetch(
                `${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/revoke-all`,
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        uid: userID,
                    }),
                }
            );
            await fetchAllSessions();
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    };

    useEffect(() => {
        fetchAllSessions();
    }, [fetchAllSessions]);

    const parser = new UAParser("user-agent");

    const sessionColumns: ColumnDef<ISession>[] = [
        {
            accessorKey: "user_agent",
            header: "User Agent",
            cell: ({ row }) => {
                parser.setUA(row.original.user_agent);
                const result = parser.getResult();

                console.log(result);

                return (
                    // render device type and browser name with logo
                    <div className="flex gap-2 h-36 items-center">
                        <div className="flex gap-2 items-center">
                            <div>
                                {result.device.type === "mobile" &&
                                    (
                                        <img
                                            src={`/user-agent/devices/phone.svg`}
                                            alt="device-logo"
                                            className="w-24 h-24"
                                        />
                                    )}
                                {result.device.type === undefined &&
                                    (
                                        <img
                                            src={`/user-agent/devices/desktop.svg`}
                                            alt="device-logo"
                                            className="w-32 h-32 text-white"
                                        />
                                    )}
                            </div>
                            <div className="flex flex-col gap-2">
                                <p className=" whitespace-nowrap">{result.device?.vendor} - {result.device?.model}</p>
                                <p className=" whitespace-nowrap">{result.os?.name} - Version: {result.os?.version ? result.os.version : "Unknown"}</p>
                                <div className=" flex gap-1 items-center">
                                    {result.browser.name?.includes("Chrome") && (
                                        <img
                                            src={`/user-agent/browsers/chrome.png`}
                                            alt="browser-logo"
                                            className="w-4 h-4"
                                        />
                                    )}
                                    {result.browser.name?.includes("Mozilla") && (
                                        <img
                                            src={`/user-agent/browsers/mozilla.png`}
                                            alt="browser-logo"
                                            className="w-4 h-4"
                                        />
                                    )}
                                    {result.browser.name?.includes("Safari") && (
                                        <img
                                            src={`/user-agent/browsers/safari.png`}
                                            alt="browser-logo"
                                            className="w-4 h-4"
                                        />
                                    )}
                                    <p className=" whitespace-nowrap">{result.browser?.name} - Version: {result.browser?.version}</p>
                                </div>
                            </div>
                        </div>

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
                        {
                            format(
                                parseInt(row.original.updated_at.$date.$numberLong)
                                , "PP - p"
                            )
                        }
                    </div>
                );
            },
        },
        {
            accessorKey: "created_at",
            header: "Expires At",
            cell: ({ row }) => {
                return (
                    <div>
                        {
                            format(addDays(
                                parseInt(row.original.created_at.$date.$numberLong)
                                , 45), "PP - p")
                        }
                    </div>
                );
            },
        },
        {
            accessorKey: "is_revoked",
            header: "Revoked",
            cell: ({ row }) => {
                return (
                    <div>
                        {row.original.is_revoked ? "Yes" : "No"}
                    </div>
                );
            },
        },
        {
            accessorKey: "action",
            header: "Action",
            cell: ({ row }) => {
                const session = row.original;
                return (
                    <div>
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <IoMdMore size={20} />
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                {!row.original.is_revoked && (
                                    <DropdownMenuItem
                                        asChild
                                        className="hover:bg-accent hover:cursor-pointer relative z-50"
                                    >
                                        <AlertDialog>
                                            <AlertDialogTrigger className="relative flex items-center w-32 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent cursor-pointer">
                                                Revoke
                                            </AlertDialogTrigger>
                                            <AlertDialogContent>
                                                <AlertDialogHeader>
                                                    <AlertDialogTitle>
                                                        Are you absolutely sure?
                                                    </AlertDialogTitle>
                                                    <AlertDialogDescription>
                                                        This action cannot be undone.
                                                    </AlertDialogDescription>
                                                </AlertDialogHeader>
                                                <AlertDialogFooter>
                                                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                    <Button
                                                        variant="destructive"
                                                        onClick={async () => {
                                                            await revokeSession(session.session_id);
                                                            await fetchAllSessions();
                                                        }}
                                                    >
                                                        {loading ? <Loader /> : <h1>Continue</h1>}
                                                    </Button>
                                                </AlertDialogFooter>
                                            </AlertDialogContent>
                                        </AlertDialog>
                                    </DropdownMenuItem>
                                )}
                                <DropdownMenuItem
                                    asChild
                                    className="hover:bg-accent hover:cursor-pointer relative z-50"
                                >
                                    <AlertDialog>
                                        <AlertDialogTrigger className="relative flex items-center w-32 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent cursor-pointer">
                                            Delete
                                        </AlertDialogTrigger>
                                        <AlertDialogContent>
                                            <AlertDialogHeader>
                                                <AlertDialogTitle>
                                                    Are you absolutely sure?
                                                </AlertDialogTitle>
                                                <AlertDialogDescription>
                                                    This action cannot be undone.
                                                </AlertDialogDescription>
                                            </AlertDialogHeader>
                                            <AlertDialogFooter>
                                                <AlertDialogCancel>Cancel</AlertDialogCancel>
                                                <Button
                                                    variant="destructive"
                                                    onClick={async () => {
                                                        await deleteSession(session.session_id);
                                                        await fetchAllSessions();
                                                    }}
                                                >
                                                    {loading ? <Loader /> : <h1>Continue</h1>}
                                                </Button>
                                            </AlertDialogFooter>
                                        </AlertDialogContent>
                                    </AlertDialog>
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                );
            },
        },
    ];

    return (
        <Card className="mt-10">
            <CardHeader>
                <CardTitle className="flex justify-between items-center">
                    <h1>Sessions</h1>
                    <DropdownMenu>
                        <DropdownMenuTrigger>
                            <Button variant="outline">
                                <IoMdMore className=" rotate-90" size={20} />
                            </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuItem
                                asChild
                                className="hover:bg-accent hover:cursor-pointer"
                            >
                                <AlertDialog>
                                    <AlertDialogTrigger className="relative flex items-center w-full rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent cursor-pointer">
                                        Revoke All
                                    </AlertDialogTrigger>
                                    <AlertDialogContent>
                                        <AlertDialogHeader>
                                            <AlertDialogTitle>
                                                Want to revoke all the sessions?
                                            </AlertDialogTitle>
                                            <AlertDialogDescription>
                                                This action cannot be undone.
                                            </AlertDialogDescription>
                                        </AlertDialogHeader>
                                        <AlertDialogFooter>
                                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                                            <Button
                                                variant="destructive"
                                                onClick={async () => {
                                                    await revokeAllSessions();
                                                }}
                                            >
                                                {loading ? <Loader /> : <h1>Continue</h1>}
                                            </Button>
                                        </AlertDialogFooter>
                                    </AlertDialogContent>
                                </AlertDialog>
                            </DropdownMenuItem>
                            <DropdownMenuItem
                                asChild
                                className="hover:bg-accent hover:cursor-pointer"
                            >
                                <AlertDialog>
                                    <AlertDialogTrigger className="relative flex items-center w-full rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground hover:bg-accent cursor-pointer">
                                        Delete All
                                    </AlertDialogTrigger>
                                    <AlertDialogContent>
                                        <AlertDialogHeader>
                                            <AlertDialogTitle>
                                                Want to delete all the sessions?
                                            </AlertDialogTitle>
                                            <AlertDialogDescription>
                                                This action cannot be undone.
                                            </AlertDialogDescription>
                                        </AlertDialogHeader>
                                        <AlertDialogFooter>
                                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                                            <Button
                                                variant="destructive"
                                                onClick={async () => {
                                                    await deleteAllSessions();
                                                }}
                                            >
                                                {loading ? <Loader /> : <h1>Continue</h1>}
                                            </Button>
                                        </AlertDialogFooter>
                                    </AlertDialogContent>
                                </AlertDialog>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </CardTitle>
            </CardHeader>
            <CardContent>
                <DataTable data={sessions ? sessions : []} columns={sessionColumns} />
            </CardContent>
        </Card>
    );
};

export default SessionTable;
