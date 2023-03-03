import { COAInsert } from "@/bindings/COAInsert";
import { COATypeView } from "@/bindings/COATypeView";
import api from "@/utils/api";
import setChange from "@/utils/setChange";
import useUsers from "@/utils/useUsers";
import { FormEvent, useEffect, useState } from "react";
import { toast } from "react-toastify";

const COAPage = () => {
  const { user, coa, router } = useUsers();
  const [types, setTypes] = useState<COATypeView>();
  const [istext, setIstext] = useState(false);
  const [coaForm, setCoaForm] = useState<COAInsert>({
    coa_tdesc: "",
    coa_sreason: "",
    coa_semp: user.Emp_Id,
    coa_stype: "",
    details: []
  })

  async function get_types() {
    try {
      const type = await api.get("/coa/types/");

      setTypes(type.data);

    } catch (e: any) {
      toast.error((e.response && e.response.data && e.response.data.err_msg) || e.message);
    }
  }


  async function updateCOA() {
    try {

      await api.put("/coa", { ...coaForm, coa_sid: coa.coa_sid });

      toast.success("Succesully updated a Certificate Of Attendance");
      router.push("/");

    } catch (e: any) {
      toast.error(e.response.data.err_msg || e.message);
    }
  }

  async function postCOA() {
    try {

      await api.post("/coa", coaForm);

      toast.success("Succesully applied a Certificate Of Attendance");
      router.push("/");

    } catch (e: any) {
      toast.error(e.response.data.err_msg || e.message);
    }
  }


  function changeDetail(index: number, name: string, value: string) {
    let new_details = coaForm.details.map((detail, i) => i === index ? { ...detail, [name]: value } : { ...detail });

    setCoaForm((prev) => ({
      ...prev,
      details: new_details
    }))
  }

  useEffect(() => {
    if (coa.coa_sid) {
      setCoaForm({
        coa_tdesc: coa.coa_stype === "C3" ? coa.coa_stypedetail : coa.coa_tdesc,
        coa_sreason: coa.coa_sreason,
        coa_semp: user.Emp_Id,
        coa_stype: coa.coa_stype,
        details: coa.details,
      })

      setIstext(coa.coa_stype === "C3" ? true : false)
    }
  }, [coa])


  useEffect(() => {
    get_types()
  }, [])

  function handleSubmit(e: FormEvent) {
    e.preventDefault();

    if (!coaForm.details || !coaForm.coa_sreason || !coaForm.coa_tdesc || !coaForm.coa_stype || !coaForm.coa_semp || coaForm.details.length <= 0) {
      toast.error("Missing Fields");
      return;
    }

    const check = coaForm.details.map((detail) => {
      if (!detail.coa_dtime || !detail.coa_ddate || !detail.coa_dtype) {
        return false
      }
      return true
    })

    if (check.includes(false)) {
      toast.error("Missing Fields");
      return
    }

    if (!coa.coa_sid) {
      postCOA();
    } else {
      updateCOA();
    }
  }

  return (
    <div className="flex justify-center">
      <div className="bg-white h-full p-7 mt-6 rounded-sm shadow-xl w-[80%] lg:w-[50rem]">
        <div className="min-h-[3rem] flex items-center">
          <h1 className="font-semibold text-xl">Certificate of Attendance Form</h1>
        </div>
        <div className="divider"></div>

        <form className="flex justify-start" onSubmit={handleSubmit}>

          <div className="flex flex-col gap-8 pr-12 w-[40%]">
            <div className="flex flex-col text-sm">
              <h1>Type:</h1>
              <select className="select w-full" value={coaForm.coa_stype} name="coa_stype" onChange={(e) => {
                setCoaForm((prev) => ({
                  ...prev,
                  coa_stype: e.target.value
                }))

                setCoaForm((prev) => ({
                  ...prev,
                  coa_tdesc: types && e.target.value !== "C3" ? types.types.filter((type) => (type.coa_tid === e.target.value))[0].coa_tdesc : ""
                }))

                setIstext(e.target.value === "C3" ? true : false)

              }}>
                <option disabled selected value=""></option>
                {types && types.types.map(type => (
                  <option key={type.coa_tid} value={type.coa_tid}> {type.coa_tdesc}</option>
                ))}
              </select>
            </div>

            {istext ? (
              <div className="flex flex-col text-sm w-full">
                <h1>Description:</h1>
                <input type="text" className="input input-ghost !rounded-none !border-[1px] !border-gray-400 ![--tw-border-opacity:1]" placeholder="Reasons..." name="coa_tdesc" value={coaForm.coa_tdesc} onChange={(e) => setChange(e, setCoaForm)} />
              </div>
            ) : <></>
            }

            <div className="flex flex-col text-sm w-full">
              <h1>Reason:</h1>
              <textarea className="textarea textarea-ghost !rounded-none !border-[1px] !border-gray-400 ![--tw-border-opacity:1]" placeholder="Reasons..." name="coa_sreason" value={coaForm.coa_sreason} onChange={(e) => setChange(e, setCoaForm)} />
            </div>
          </div>

          <div className="divider divider-horizontal"></div>

          <div className="w-[60%] relative">

            <div className="flex justify-start">
              <h1 className="w-full text-sm font-semibold">Type</h1>
              <h1 className="w-full text-sm font-semibold">Date</h1>
              <h1 className="w-full text-sm font-semibold">Time</h1>
            </div>

            <div className="divider !my-0 !min-h-[1.5px]"></div>

            <div className="flex flex-col gap-4">
              {coaForm.details.map((detail, i) => (
                <div className="flex gap-3" key={i}>
                  <select className="select select-bordered w-[20%]" value={detail.coa_dtype} onChange={(e) => changeDetail(i, "coa_dtype", e.target.value)} >
                    <option value="" disabled selected></option>
                    <option value="I">In</option>
                    <option value="O">Out</option>
                  </select>

                  <input type="date" name="date_from" className="input input-ghost !px-0 !rounded-none !border-r-0 !border-l-0 !border-t-0 !border-b-[1px] !border-gray-400 ![--tw-border-opacity:1] w-full" value={detail.coa_ddate} onChange={(e) => changeDetail(i, "coa_ddate", e.target.value)} />

                  <input type="time" name="date_from" className="input input-ghost !px-0 !rounded-none !border-r-0 !border-l-0 !border-t-0 !border-b-[1px] !border-gray-400 ![--tw-border-opacity:1] w-full" value={detail.coa_dtime} onChange={(e) => changeDetail(i, "coa_dtime", e.target.value + ":00")} />
                </div>
              ))}
            </div>

            <div className="flex flex-col gap-2 mt-5 items-end">
              <button type="button" className="w-max btn gap-2 btn-ghost !h-[2rem] !min-h-[10px]" onClick={() => {
                setCoaForm((prev) => ({
                  ...prev,
                  details: [
                    ...prev.details,
                    {
                      coa_dtype: "",
                      coa_dtime: "",
                      coa_ddate: ""
                    }
                  ]
                }))
              }}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                Add
              </button>
              <button type="submit" className="btn w-max">Submit</button>
            </div>
          </div>
        </form>
      </div >
    </div >
  )
}

export default COAPage;
