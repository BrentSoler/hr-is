import { ChangeEvent, Dispatch, SetStateAction } from "react";

type inputs = HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;

export default function setChange<T>(e: ChangeEvent<inputs>, callback: Dispatch<SetStateAction<T>>) {
    callback((prevData: T) => ({
        ...prevData,
        [e.target.name]: e.target.value
    }));
}
