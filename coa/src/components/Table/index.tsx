import { COAView } from "@/bindings/COAView";
import api from "@/utils/api";
import useUsers from "@/utils/useUsers";
import { Tab } from "@headlessui/react";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";
import TableItem from "../TableItem";

const Table: React.FC = () => {
    const { user, router } = useUsers();
    const [coa, setCoa] = useState<COAView>();

    async function get_credits() {
        try {
            const coas = await api.get(`/coa/${user.Emp_Id}`);

            setCoa(coas.data);
        } catch (e: any) {
            toast.error(e.message)
        }
    }

    useEffect(() => {
        if (!user.Emp_Id) {
            router.push("/NoToken");
        }

        get_credits();
    }, [user])

    return (
        <Tab.Group>
            <Tab.List className="flex justify-end p-2 ">
                <Tab className={({ selected }) => `btn btn-ghost gap-2 !min-h-[2.5rem] !h-[2.5rem] !rounded-none 
                    ${selected ? "!font-bold !border-b-black !border-b-[3px]" : "!font-light"}`}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                    </svg>
                    Pending/Resubmitted for editing
                </Tab>
                <Tab className={({ selected }) => `btn btn-ghost gap-2 !min-h-[2.5rem] !h-[2.5rem] !rounded-none 
                    ${selected ? "!font-bold !border-b-black !border-b-[3px]" : "!font-light"}`}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                    </svg>
                    Approved
                </Tab>
                <Tab className={({ selected }) => `btn btn-ghost gap-2 !min-h-[2.5rem] !h-[2.5rem] !rounded-none 
                    ${selected ? "!font-bold !border-b-black !border-b-[3px]" : "!font-light"}`}
                >
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                    Rejected/Cancelled
                </Tab>
            </Tab.List>
            <Tab.Panels>
                <Tab.Panel>
                    {coa &&
                        <TableItem refresh={get_credits} COA={{ coa: coa.coa.filter(coa => coa.coa_sstatus === 0) }} type="P" />
                    }
                </Tab.Panel>
                <Tab.Panel>
                    {coa &&
                        <TableItem refresh={get_credits} COA={{ coa: coa.coa.filter(coa => coa.coa_sstatus === 1) }} type="A" />
                    }
                </Tab.Panel>
                <Tab.Panel>
                    {coa &&
                        <TableItem refresh={get_credits} COA={{ coa: coa.coa.filter(coa => coa.coa_sstatus === 2) }} type="D" />
                    }
                </Tab.Panel>
            </Tab.Panels>
        </Tab.Group>
    )
}

export default Table;
