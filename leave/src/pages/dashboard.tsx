import { EmployeeInfo } from "@/bindings/EmployeeInfo";
import Webcard from "@/components/Webcard";
import api from "@/utils/api";
import useUserStore from "@/utils/store";
import Link from "next/link";
import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";

const Dashboard: React.FC = () => {
  const token = useUserStore((state: any) => state.token);
  const [userInfo, setUserInfo] = useState<EmployeeInfo>();
  const router = useRouter();

  async function get_info() {
    try {
      const user = await api.get(`/${token}`);

      setUserInfo(user.data);
    } catch (error: any) {
      toast.error((error.response && error.response.data && error.response.data.err_msg) ||
        error.message);
    }
  }

  useEffect(() => {
    if (!token) {
      router.push("/");
    }

    get_info();
  }, [token])

  return (
    <div className="h-screen flex flex-col items-center mt-5 gap-8">
      {userInfo &&
        <>
          <h1 className="font-bold text-4xl">Welcome, {userInfo.Employee.Emp_Last}!</h1>
          <div className="flex gap-3 w-[100%] lg:w-[60%] justify-center flex-wrap">
            {userInfo.Access.map(module => (
              <Webcard link={module.Mnu_Http ? module.Mnu_Http : ""} name={module.Mnu_Desc} />
            ))}
          </div>
        </>
      }
    </div>
  )
}

export default Dashboard;
