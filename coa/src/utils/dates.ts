import { Schedule } from "@/bindings/Schedule";

export default function getDates(date_from: Date, date_to: Date, schedule: Schedule[]) {
    const date_arr = new Array();
    const sched = schedule.map(sch => sch.Sch_Day.toLowerCase());
    let current = date_from;

    while (current <= date_to) {
        const day = new Date(current).toLocaleDateString('en-US', { weekday: 'long' }).toLowerCase();
        if (!sched.includes(day)) {
            date_arr.push(current.toISOString().split("T")[0]);
        }
        current = addDay(current);
    }

    console.log(date_arr)
    return date_arr;
}

function addDay(date: Date) {
    const date_parsed = new Date(date);
    date_parsed.setDate(date_parsed.getDate() + 1);

    return date_parsed;
}
