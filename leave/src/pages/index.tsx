import { LeaveDetail } from "@/bindings/LeaveDetail";
import Table from "@/components/Table";
import useUserStore from "@/utils/store";
import useUsers from "@/utils/useUsers";
import { Dialog, Transition } from "@headlessui/react";
import Link from "next/link";
import { Fragment, useState } from "react";

export default function LoginPage() {
  const { user, leaves } = useUsers();
  const setLeave = useUserStore((state: any) => state.setLeave);
  const [details, setDetails] = useState<LeaveDetail[]>([]);

  return (
    <div className="flex justify-center">
      <div className="bg-white h-full p-7 mt-6 rounded-sm shadow-xl w-[80%] lg:w-[50rem]">
        <div className="flex justify-between items-center">
          <h1 className="font-semibold  text-xl">Welcome, {user.Emp_Last}!</h1>
          <Link href="/leave">
            <button className="btn-ghost btn gap-2" onClick={() => setLeave({})} >
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
              Add
            </button>
          </Link>
        </div>
        <div className="divider"></div>
        {leaves && leaves.Leaves.map((leave) => (
          <div className="flex justify-between">
            <h1 className="text-sm">{leave.Lev_Desc}</h1>
            <h1 className="text-sm font-bold">{leave.Availleave}</h1>
          </div>
        ))}
        <div className="divider"></div>
        <Table cb={setDetails} />

        <Transition appear show={details.length > 0} as={Fragment}>
          <Dialog as="div" className="relative z-10" onClose={() => setDetails([])}>
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0"
              enterTo="opacity-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100"
              leaveTo="opacity-0"
            >
              <div className="fixed inset-0 bg-black bg-opacity-25" />
            </Transition.Child>

            <div className="fixed inset-0 overflow-y-auto">
              <div className="flex min-h-full items-center justify-center p-4 text-center">
                <Transition.Child
                  as={Fragment}
                  enter="ease-out duration-300"
                  enterFrom="opacity-0 scale-95"
                  enterTo="opacity-100 scale-100"
                  leave="ease-in duration-200"
                  leaveFrom="opacity-100 scale-100"
                  leaveTo="opacity-0 scale-95"
                >
                  <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all">
                    <Dialog.Title
                      as="h3"
                      className="text-lg font-medium leading-6 text-gray-900"
                    >
                      Leave Details
                    </Dialog.Title>
                    {details.map(date => (
                      <div className="py-2 flex items-center justify-between border-t-[1.5px]">
                        <p className="text-sm text-gray-500">
                          {date.Lea_Ddate}
                        </p>
                        <p className="text-sm text-gray-500">
                          {date.Lea_Dtype === "H" ? "Half Day" : "Whole Day"}
                        </p>
                        {date.Lea_Dtype === "H" &&
                          <p className="text-sm text-gray-500">
                            {date.Lea_Dampm === "A" ? "A.M" : "P.M"}
                          </p>
                        }
                      </div>

                    ))}

                    <div className="mt-4">
                      <button
                        type="button"
                        className="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
                        onClick={() => setDetails([])}
                      >
                        Ok
                      </button>
                    </div>
                  </Dialog.Panel>
                </Transition.Child>
              </div>
            </div>
          </Dialog>
        </Transition>
      </div>
    </div>
  )
}
