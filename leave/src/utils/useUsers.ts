import { AvailableLeave } from "@/bindings/AvailableLeave";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";
import api from "./api";
import useUserStore from "./store";

export default function useUsers() {
    const user = useUserStore((state: any) => state.user);
    const leave = useUserStore((state: any) => state.leave);
    const sched = useUserStore((state: any) => state.sched);
    const [leaves, setLeaves] = useState<AvailableLeave>();
    const router = useRouter();

    async function get_credits() {
        try {
            const leaves = await api.get(`/leave/credits/${user.Emp_Id}`);

            setLeaves(leaves.data);
        } catch (error: any) {
            toast.error((error.response && error.response.data && error.response.data.err_msg) ||
                error.message);
        }
    }

    useEffect(() => {
        if (!user.Emp_Id) {
            router.push("/NoToken");
        }

        get_credits();
    }, [user])

    return {
        user: user,
        schedule: sched,
        router,
        leaves,
        leave
    }
}
