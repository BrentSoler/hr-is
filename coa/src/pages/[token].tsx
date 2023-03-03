import api from "@/utils/api";
import useUserStore from "@/utils/store";
import { NextPage } from "next";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";

const ValidationPage: NextPage = () => {
  const setUser = useUserStore((state: any) => state.setUser);
  const setSched = useUserStore((state: any) => state.setSched);
  const [state, setState] = useState("Validating token");
  const router = useRouter();
  const { token } = router.query;

  async function get_info() {
    try {
      const user = await api.get(`/${token}`);

      setUser(user.data.Employee);
      setSched(user.data.Schedules);

      router.push("/")
    } catch (e: any) {
      setState("Error Validating Token");
      toast.error((e.response && e.response.data && e.response.data.err_msg) || e.message);
    }
  }


  useEffect(() => {
    if (token) {
      get_info();
    }

  }, [token])

  return (
    <div className="h-screen grid place-items-center">
      <h1 className="font-bold text-3xl">{state}</h1>
    </div>
  )
}

export default ValidationPage;
