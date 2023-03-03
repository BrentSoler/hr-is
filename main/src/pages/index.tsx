import { EmployeeLogin } from "@/bindings/EmployeeLogin"
import api from "@/utils/api";
import setChange from "@/utils/setChange"
import useUserStore from "@/utils/store";
import { useRouter } from "next/router";
import { FormEvent, useState } from "react"
import { toast } from "react-toastify";

export default function LoginPage() {
  const setToken = useUserStore((state: any) => state.setToken);
  const [loginData, setLoginData] = useState<EmployeeLogin>({
    Emp_Id: "",
    Emp_Pswd: "",
  })
  const router = useRouter();

  async function login(loginForm: typeof loginData) {
    try {
      const user = await api.post("/login", loginForm);


      return user.data;
    } catch (error: any) {
      toast.error((error.response && error.response.data && error.response.data.err_msg) ||
        error.message);
    }
  }


  async function onSubmit(e: FormEvent) {
    e.preventDefault();

    const token = await login(loginData);

    if (token) {
      setToken(token.Token);

      router.push("/dashboard");
    }
  }

  return (
    <div className="h-screen flex justify-between">
      <div className=" w-[40%] flex flex-col justify-center p-7 gap-5">
        <h1 className="font-bold text-3xl ">Login</h1>
        <form className="flex flex-col gap-4" onSubmit={onSubmit}>

          <input type="text" className="input !px-0 !rounded-none !border-r-0 !border-l-0 !border-t-0 !border-b-[1px] !border-gray-400 ![--tw-border-opacity:1]" placeholder="Username..." name="Emp_Id" onChange={(e) => setChange(e, setLoginData)} value={loginData.Emp_Id} />

          <input type="password" className="input !px-0 !rounded-none !border-r-0 !border-l-0 !border-t-0 !border-b-[1px] !border-gray-400 ![--tw-border-opacity:1]" placeholder=" Password..." name="Emp_Pswd" onChange={(e) => setChange(e, setLoginData)} value={loginData.Emp_Pswd} />

          <button className="btn !rounded-sm mt-7" type="submit">Login</button>
        </form>
      </div>

      <div className="shadow-xl w-[100%] bg-white rounded-3xl"></div>
    </div>
  )
}
