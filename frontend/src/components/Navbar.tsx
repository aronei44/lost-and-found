"use client";
import { useGlobalContext } from "@/hooks/globalprovider";
import NavButton from "./NavButton";

const Navbar = () => {
    const { 
        action: {
            setActiveState,
            setLogout
        },
        state: {
            authenticated
        }
    } = useGlobalContext();
    return (
        <nav className="w-full bg-slate-400 p-4">
            <div className="container mx-auto flex justify-between items-center">
                <NavButton
                    onClick={() => setActiveState("main")}
                    label="Cari Dan Kenali"
                />
                <div>
                    {!authenticated ? (
                        <>
                        <NavButton
                            onClick={() => setActiveState("login")}
                            label="Masuk"
                        />
                        <NavButton
                            onClick={() => setActiveState("register")}
                            label="Daftar"
                        />
                        </>
                    ): (
                        <>
                            <NavButton
                                onClick={() => setActiveState("lost_people")}
                                label="Daftar Dicari"
                            />
                            <NavButton
                                onClick={() => setActiveState("lost_people_monitored")}
                                label="Daftar Dicari (Monitor)"
                            />
                            <NavButton
                                onClick={() => setActiveState("profile")}
                                label="Profil"
                            />
                            <NavButton
                                onClick={() => setLogout()}
                                label="Keluar"
                            />
                        </>
                    )}
                </div>
            </div>
        </nav>
    );
}
  
export default Navbar;