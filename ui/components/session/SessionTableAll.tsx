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
import { DataTable } from "../ui/data-table";
import { addDays, format } from "date-fns";

const SessionTableAll: React.FC = () => {
    const [loading, setLoading] = React.useState(false);
    const [sessions, setSessions] = React.useState<ISession[]>([]);

    // fetch all sessions
    const fetchAllSessions = useCallback(async () => {
        try {
            setLoading(true);
            const res = await fetch(
                `${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/get-all`,
                {
                    method: "GET",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    cache: "no-cache",
                }
            );
            const { data } = await res.json();
            setSessions(data);
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    }, []);

    // revoke session function
    const revokeSession = async (session_id: string, uid: string) => {
        try {
            setLoading(true);
            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/revoke`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    session_id,
                    uid,
                }),
            });
            await fetchAllSessions();
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    };

    // delete session function
    const deleteSession = async (session_id: string, uid: string) => {
        try {
            setLoading(true);
            await fetch(`${process.env.NEXT_PUBLIC_ENDPOINT}/api/session/delete`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    session_id,
                    uid,
                }),
            });
            await fetchAllSessions();
        } catch (error) {
            console.error("Error during POST request:", error);
        }
        setLoading(false);
    };

    useEffect(() => {
        fetchAllSessions();
    }, [fetchAllSessions]);

    const sessionColumns: ColumnDef<ISession>[] = [
        {
            accessorKey: "user_agent",
            header: "User Agent",
            cell: ({ row }) => {
                const session = row.original;
                return (
                    // render device type and browser name with logo
                    <div className="flex gap-2 h-36 items-center">
                        <div className="flex gap-2 items-center">
                            <div>
                                {session.device === "smartphone" &&
                                    (
                                        <img
                                            src={`/user-agent/devices/phone.svg`}
                                            alt="device-logo"
                                            className="w-24 h-24"
                                        />
                                    )}
                                {session.device === "pc" &&
                                    (
                                        <img
                                            src={`/user-agent/devices/desktop.svg`}
                                            alt="device-logo"
                                            className="w-24 h-24 text-white"
                                        />
                                    )}
                            </div>
                            <div className="flex flex-col gap-2">
                                <p className=" whitespace-nowrap">{session?.device === "smartphone" ? "Smartphone" : session?.device === "pc" ? "Desktop" : ""} ({session.vendor})</p>
                                <p className=" whitespace-nowrap">{session?.os} - Version {session.os_version}</p>
                                <div className=" flex gap-1 items-center">
                                    {session?.browser.includes("Chrome") && (
                                        <img
                                            src={`/user-agent/browsers/chrome.png`}
                                            alt="browser-logo"
                                            className="w-4 h-4"
                                        />
                                    )}
                                    {session?.browser.includes("Mozilla") && (
                                        <img
                                            src={`/user-agent/browsers/mozilla.png`}
                                            alt="browser-logo"
                                            className="w-4 h-4"
                                        />
                                    )}
                                    {session?.browser.includes("Safari") && (
                                        <img
                                            src={`/user-agent/browsers/safari.png`}
                                            alt="browser-logo"
                                            className="w-4 h-4"
                                        />
                                    )}
                                    <p className=" whitespace-nowrap">{session?.browser} - Version: {session?.browser_version}</p>
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
                                                            await revokeSession(session.session_id, session.uid);
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
                                                        await deleteSession(session.session_id, session.uid);
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
        <DataTable data={sessions ? sessions : []} columns={sessionColumns} />
    );
};

export default SessionTableAll;
