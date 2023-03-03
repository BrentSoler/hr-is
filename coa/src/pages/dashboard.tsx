import { EmployeeInfo } from "@/bindings/EmployeeInfo";
import api from "@/utils/api";
import useUserStore from "@/utils/store";
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
    } catch (e: any) {
      toast.error(e.response.data.err_msg | e.message);
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
          <div className="flex gap-3 w-[100%] lg:w-[60%] justify-center flex-wrap">
          </div>
        </>
      }
    </div>
  )
}

export default Dashboard;
