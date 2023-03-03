import { CancelCOA } from "@/bindings/CancelCOA";
import { COAView } from "@/bindings/COAView";
import api from "@/utils/api";
import useUserStore from "@/utils/store";
import useUsers from "@/utils/useUsers";
import { toast } from "react-toastify";

const TableItem: React.FC<{ COA: COAView, type: string, refresh: () => Promise<void> }> = ({ COA, type, refresh }) => {
    const { user, router } = useUsers();
    const setCOA = useUserStore((state: any) => state.setCOA);
    const { coa } = COA;

    async function cancel(cancel_coa: CancelCOA) {
        try {

            await api.patch("/coa", cancel_coa);

            toast.success("Sucessfully cancelled");

            refresh();
        } catch (e: any) {
            toast.error((e.response && e.response.data && e.response.data.err_msg) || e.message);
        }
    }

    return (
        <div className="overflow-x-auto">
            <table className="table w-full !rounded-none">
                <thead>
                    <tr>
                        <th>Type</th>
                        <th>Date Filed</th>
                        <th>Reason</th>
                        {type === "A" && <th>Approved By</th>}
                        {type === "A" && <th>Approved Date</th>}
                        <th></th>
                    </tr>
                </thead>
                {coa.map((coa) => (
                    <tr className="hover items-center" key={coa.coa_sid}>
                        <th>
                            <div className="flex flex-col">
                                {coa.coa_stypedetail}
                                <div className={`h-full font-light badge badge-xs ${Number(coa.coa_sstatus) === 0 ? "" :
                                    Number(coa.coa_sstatus) === 1 ? "badge-success" : "badge-error"}`}>{Number(coa.coa_sstatus) === 0 ? "Pending/Resubmitted for editing" :
                                        Number(coa.coa_sstatus) === 1 ? "Approved" : "Rejected/Cancelled"}
                                </div>
                            </div>
                        </th>
                        <td>{!coa.coa_logdate ? "" : coa.coa_logdate.split("T")[0]}</td>
                        <td>{coa.coa_sreason}</td>
                        {type === "A" &&
                            <td>{coa.coa_sapprovedby}</td>
                        }
                        {type === "A" &&
                            <td>{coa.coa_sapprovedate}</td>
                        }
                        {type === "P" &&
                            <td className="flex">
                                <div className="flex gap-4">
                                    <button className="btn z-50" onClick={() => { setCOA(coa); router.push("/coa") }}>
                                        Edit
                                    </button>
                                    <button className="btn z-50" onClick={() => cancel({ emp_id: user.Emp_Id, status: 2, coa_sid: coa.coa_sid })}>
                                        Cancel
                                    </button>
                                </div>
                            </td>
                        }
                    </tr>
                ))}
                <tbody>
                </tbody>
            </table>
        </div >)
}

export default TableItem;
