import { useRouter } from "next/router";
import { useEffect } from "react";
import useUserStore from "./store";

export default function useUsers() {
    const user = useUserStore((state: any) => state.user);
    const coa = useUserStore((state: any) => state.coa);
    const router = useRouter();

    useEffect(() => {
        if (!user.Emp_Id) {
            router.push("/NoToken");
        }

        // get_credits();
    }, [user])

    return {
        user,
        coa,
        router,
    }
}
