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
                    label="Lost And Found"
                />
                <div>
                    {!authenticated ? (
                        <>
                        <NavButton
                            onClick={() => setActiveState("login")}
                            label="Login"
                        />
                        <NavButton
                            onClick={() => setActiveState("register")}
                            label="Register"
                        />
                        </>
                    ): (
                        <>
                            <NavButton
                                onClick={() => setActiveState("profile")}
                                label="Profile"
                            />
                            <NavButton
                                onClick={() => setLogout()}
                                label="Logout"
                            />
                        </>
                    )}
                </div>
            </div>
        </nav>
    );
}
  
export default Navbar;