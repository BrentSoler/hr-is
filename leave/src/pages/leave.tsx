import { GetAvailableLeaves } from "@/bindings/GetAvailableLeaves";
import { LeaveDetailInsert } from "@/bindings/LeaveDetailInsert";
import { LeaveInsert } from "@/bindings/LeaveInsert";
import { PendingLeaves } from "@/bindings/PendingLeaves";
import api from "@/utils/api";
import getDates from "@/utils/dates";
import setChange from "@/utils/setChange";
import useUsers from "@/utils/useUsers";
import { FormEvent, useEffect, useState } from "react";
import { toast } from "react-toastify";

const LeavePage = () => {
  const { router, leave, schedule, user, leaves } = useUsers();
  const [leaveForm, setLeaveForm] = useState<LeaveInsert>({
    Emp_Id: user.Emp_Id,
    Leave_Id: "",
    Reason: "",
    Date_From: "",
    Date_To: "",
    Lea_Swithpay: 0.0,
    Lea_Swithoutpay: 0.0,
    Details: []
  });
  const [credits, setCredits] = useState<PendingLeaves>({
    Credits: 0.0,
    Available: 0.0
  });

  function changeDetail(index: number, type: string) {
    let new_details = leaveForm.Details.map((detail, i) => i === index ? { ...detail, Date_Type: type, Am_Pm: type === "H" ? "A" : null } : { ...detail });

    setLeaveForm((prev) => ({
      ...prev,
      Details: new_details
    }))
  }

  function changeHalf(index: number, type: string) {
    let new_details = leaveForm.Details.map((detail, i) => i === index ? { ...detail, Am_Pm: type } : { ...detail });

    setLeaveForm((prev) => ({
      ...prev,
      Details: new_details
    }))
  }

  function dates() {
    const date_arr = getDates(new Date(leaveForm.Date_From), new Date(leaveForm.Date_To), schedule);
    const detail_arr: LeaveDetailInsert[] = new Array();

    for (const date in date_arr) {
      if (leaveForm.Details.length <= 0) {
        detail_arr.push({ Date: date_arr[date], Am_Pm: null, Date_Type: "W" });
        console.log("hit")
      } else {
        if (leaveForm.Details[date]) {
          detail_arr.push({ Date: date_arr[date], Am_Pm: leaveForm.Details[date].Date === date_arr[date] ? leaveForm.Details[date].Am_Pm : null, Date_Type: leaveForm.Details[date].Date === date_arr[date] ? leaveForm.Details[date].Date_Type : "W" });
        } else {
          detail_arr.push({ Date: date_arr[date], Am_Pm: null, Date_Type: "W" });
        }
      }
    }

    setLeaveForm((details) => ({
      ...details,
      Details: detail_arr
    }));
  }

  useEffect(() => {
    if (leave.Lea_Sid) {
      setLeaveForm(() => ({
        Date_To: leave.Lea_Sto,
        Date_From: leave.Lea_Sfrm,
        Emp_Id: user.Emp_Id,
        Reason: leave.Lea_Sreason,
        Leave_Id: leave.Lea_Stype,
        Details: leave.Details,
        Lea_Swithoutpay: leave.Lea_Swithoutpay,
        Lea_Swithpay: leave.Lea_Swithpay,
      }))

      get_pending(leave.Lea_Stype);
    }
  }, [leave, leave.Details])

  useEffect(() => {
    if (leaveForm.Date_To) {
      dates();
    }

    if (!leaveForm.Date_To && !leave.Lea_Sto) {
      setLeaveForm((details) => ({
        ...details,
        Details: []
      }));
    }
  }, [leaveForm.Date_To, leaveForm.Date_From])

  async function postLeave() {
    try {

      await api.post("/leave", leaveForm);

      toast.success("Succesully applied leave");
      router.push("/");

    } catch (error: any) {
      toast.error((error.response && error.response.data && error.response.data.err_msg) ||
        error.message);
    }
  }

  useEffect(() => {
    let match = 0;

    if (leaveForm.Details.length > 0) {
      for (const lea of leaveForm.Details) {
        if (lea.Date_Type === "H") {
          match += 0.5;
        } else {
          match += 1;
        }
      }


      setLeaveForm((prev) => ({
        ...prev,
        Lea_Swithpay: match < credits.Available ? match : credits.Available,
        Lea_Swithoutpay: match > credits.Available ? match - credits.Available : 0
      }))
    }

  }, [credits, leaveForm.Details])

  async function get_pending(leave_id: string) {
    try {

      const avail_form: GetAvailableLeaves = {
        Emp_Id: user.Emp_Id,
        Leave_Type: leave_id
      };

      const pending = await api.post("/leave/pending", avail_form);

      let match = 0;

      if (leave.Lea_Sid) {
        for (const lea of leave.Details) {
          if (lea.Date_Type === "H") {
            match += 0.5;
          } else {
            match += 1;
          }
        }
      }

      setCredits({
        Credits: pending.data.Credits,
        Available: leave.Lea_Sid && leave.Lea_Stype === leave_id ? pending.data.Available + match : pending.data.Available
      })
    } catch (error: any) {
      toast.error((error.response && error.response.data && error.response.data.err_msg) ||
        error.message);
    }
  }

  function submitHandler(e: FormEvent) {
    e.preventDefault();
    if (leaveForm.Details.length === 0 || !leaveForm.Leave_Id || !leaveForm.Reason) {
      toast.error("Missing Feilds");
      return;
    }

    if (!leave.Lea_Sid) {
      postLeave();
    } else {
      updateLeave();
    }
  }


  async function updateLeave() {
    try {

      await api.put("/leave", { ...leaveForm, Lea_Sid: leave.Lea_Sid });

      toast.success("Succesully applied leave");
      router.push("/");

    } catch (error: any) {
      toast.error((error.response && error.response.data && error.response.data.err_msg) ||
        error.message);
    }
  }
  return (
    <div className="flex justify-center">
      <div className="bg-white h-full p-7 mt-6 rounded-sm shadow-xl w-[80%] lg:w-[50rem]">
        <div className="min-h-[3rem] flex items-center">
          <h1 className="font-semibold text-xl">Leave Application Form</h1>
        </div>
        <div className="divider"></div>

        <div className="flex justify-start">
          <div className="flex flex-col gap-8 pr-12">
            <h1>
              <span className="font-bold">Fullname: </span>
              {user.Emp_Last},{user.Emp_First} {user.Emp_Mid}
            </h1>
            <h1>
              <span className="font-bold">Available Leave Credits: </span>
              {credits.Available}
            </h1>
            <h1>
              <span className="font-bold">Current Leave Credits: </span>
              {credits.Credits}
            </h1>
            <h1>
              <span className="font-bold">Days With Pay: </span>
              {leaveForm.Lea_Swithpay}
            </h1>
            <h1>
              <span className="font-bold">Days Without Pay: </span>
              {leaveForm.Lea_Swithoutpay}
            </h1>
          </div>

          <div className="divider divider-horizontal"></div>

          <form className="w-[70%] flex flex-col gap-5" onSubmit={submitHandler}>
            <div>

              <div className="flex flex-col text-sm">
                <h1>Leave Type:</h1>
                <select name="Leave_Id" className="select w-full max-w-xs" value={leaveForm.Leave_Id} onChange={(e) => {
                  setChange(e, setLeaveForm)
                  get_pending(e.target.value)
                }}>
                  <option disabled selected value="">Leave Type</option>
                  {leaves && leaves.Leaves.map((leave) => (
                    <option value={leave.Eml_Leave} key={leave.Eml_Leave}>{leave.Lev_Desc}</option>
                  ))}
                </select>
              </div>

            </div>

            <div className="flex gap-3">
              <div className="flex flex-col text-sm w-full">

                <h1>From:</h1>
                <input type="date" name="Date_From" className="input input-ghost !px-0 !rounded-none !border-r-0 !border-l-0 !border-t-0 !border-b-[1px] !border-gray-400 ![--tw-border-opacity:1]" onChange={(e) => setChange(e, setLeaveForm)} value={leaveForm.Date_From} placeholder="Leave Start Date" />
              </div>

              <div className="flex flex-col text-sm w-full">
                <h1>To:</h1>
                <input type="date" name="Date_To" className="input input-ghost !px-0 !rounded-none !border-r-0 !border-l-0 !border-t-0 !border-b-[1px] !border-gray-400 ![--tw-border-opacity:1]" onChange={(e) => setChange(e, setLeaveForm)} value={leaveForm.Date_To} placeholder="Leave End Date" disabled={leaveForm.Date_From ? false : true} min={leaveForm.Date_From} />
              </div>
            </div>

            <div className="flex gap-3">
              <div className="flex flex-col text-sm w-full">
                <h1>Reason:</h1>
                <textarea name="Reason" className="textarea textarea-ghost !rounded-none !border-[1px] !border-gray-400 ![--tw-border-opacity:1]" placeholder="Reasons..." onChange={(e) => setChange(e, setLeaveForm)} value={leaveForm.Reason} />
              </div>
            </div>

            <div className="divider !my-0"></div>

            <div>
              <h1 className="font-bold text-lg ">Date Breakdown:</h1>
              {leaveForm.Details.map((date, i) => (
                <div className="flex justify-between py-3 border-b-[1.5px] items-center" key={i}>
                  <h1>{date.Date}</h1>

                  <div>
                    <div className="flex gap-5">
                      <div className="flex flex-col items-center">
                        <h1 className="text-sm font-semibold">Whole Day</h1>
                        <input type="radio" name={date.Date} className="radio radio-xs" value={date.Date_Type} checked={date.Date_Type === "W"} onChange={(_) => changeDetail(i, "W")} />
                      </div>
                      <div className="flex flex-col items-center">
                        <h1 className="text-sm font-semibold">Half Day</h1>
                        <input type="radio" name={date.Date} className="radio radio-xs" value={date.Date_Type} checked={date.Date_Type === "H"} onChange={(_) => { changeDetail(i, "H") }} />
                      </div>
                    </div>
                  </div>

                  {date.Date_Type === "H" &&
                    <div>
                      <div className="flex gap-5">
                        <div className="flex flex-col items-center">
                          <h1 className="text-sm font-semibold">A.M</h1>
                          <input type="radio" name={date.Date + "half"} className="radio radio-xs" value={date.Am_Pm ? date.Am_Pm : ""} checked={date.Am_Pm === "A"} onChange={(_) => changeHalf(i, "A")} />
                        </div>
                        <div className="flex flex-col items-center">
                          <h1 className="text-sm font-semibold">P.M</h1>
                          <input type="radio" name={date.Date + "half"} className="radio radio-xs" value={date.Am_Pm ? date.Am_Pm : ""} checked={date.Am_Pm === "P"} onChange={(_) => changeHalf(i, "P")} />
                        </div>
                      </div>
                    </div>}
                </div>
              ))}
            </div>

            <div className="flex justify-end">
              <button className="btn" type="submit">Apply</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  )
}

export default LeavePage;
