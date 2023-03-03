import Table from "@/components/Table";
import useUserStore from "@/utils/store";
import useUsers from "@/utils/useUsers";
import Link from "next/link";

export default function LoginPage() {
  const { user } = useUsers();
  const setCOA = useUserStore((state: any) => state.setCOA);

  return (
    <div className="flex justify-center">
      <div className="bg-white h-full p-7 mt-6 rounded-sm shadow-xl w-[80%] lg:w-[50rem]">
        <div className="flex justify-between items-center">
          <h1 className="font-semibold  text-xl">Welcome, {user.Emp_Last}!</h1>
          <Link href="/coa">
            <button className="btn-ghost btn gap-2" onClick={() => setCOA({})}>
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
              Add
            </button>
          </Link>
        </div>
        <div className="divider"></div>

        <Table />

      </div>
    </div>
  )
}
