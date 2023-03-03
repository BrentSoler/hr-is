import { CancelLeave } from "@/bindings/CancelLeave";
import { LeaveDetail } from "@/bindings/LeaveDetail";
import { LeaveView } from "@/bindings/LeaveView";
import api from "@/utils/api";
import useUserStore from "@/utils/store";
import useUsers from "@/utils/useUsers";
import { Dispatch, SetStateAction } from "react";
import { toast } from "react-toastify";

const TableItem: React.FC<{ Leave: LeaveView, type: string, cb: Dispatch<SetStateAction<LeaveDetail[]>>, refresh: () => Promise<void> }> = ({ Leave, type, cb, refresh }) => {
    const { user, router } = useUsers();
    const setLeave = useUserStore((state: any) => state.setLeave);
    const { Leaves } = Leave;

    async function cancel(cancel_leave: CancelLeave) {
        try {

            await api.patch("/leave", cancel_leave);

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
                        <th>Leave Type</th>
                        <th>Leave From</th>
                        <th>Leave To</th>
                        <th>Reasons</th>
                        <th>Days With Pay</th>
                        <th>Days Without Pay</th>
                        {type === "A" && <th>Approved By</th>}
                        {type === "A" && <th>Approved Date</th>}
                        <th></th>
                    </tr>
                </thead>
                {Leaves.map((leave) => (
                    <tr className="hover" onClick={() => cb(leave.Details)}>
                        <th>
                            <div className="flex flex-col gap-2">
                                {leave.Lev_Desc}
                                <div className={`font-light badge badge-xs ${Number(leave.Lea_Sstatus) === 0 ? "" :
                                    Number(leave.Lea_Sstatus) === 1 ? "badge-success" : "badge-error"}`}>{Number(leave.Lea_Sstatus) === 0 ? "Pending/Resubmitted for editing" :
                                        Number(leave.Lea_Sstatus) === 1 ? "Approved" : "Rejected/Cancelled"}
                                </div>
                            </div>
                        </th>
                        <td>{leave.Lea_Sfrm}</td>
                        <td>{leave.Lea_Sto}</td>
                        <td>{leave.Lea_Sreason}</td>
                        <td>{leave.Lea_Swithpay}</td>
                        <td>{leave.Lea_Swithoutpay}</td>
                        {type === "A" &&
                            <td>{leave.Lea_Sapprovedby}</td>
                        }
                        {type === "A" &&
                            <td>{leave.Lea_Sapprovedate}</td>
                        }
                        {type === "P" &&
                            <td>
                                <div className="flex gap-4">
                                    <button className="btn" onClick={() => {
                                        setLeave({
                                            ...leave,
                                            Details: leave.Details.map((leave: LeaveDetail) => {
                                                return {
                                                    Date_Type: leave.Lea_Dtype,
                                                    Am_Pm: leave.Lea_Dampm,
                                                    Date: leave.Lea_Ddate
                                                }
                                            }).sort((a, b) => a.Date.localeCompare(b.Date))
                                        });
                                        router.push("/leave");
                                    }}>
                                        Edit
                                    </button>
                                    <button className="btn" onClick={() => {
                                        cancel({ Emp_Id: user.Emp_Id, Status: 2, Leave_Id: leave.Lea_Sid })
                                        cb([])
                                    }}>
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
