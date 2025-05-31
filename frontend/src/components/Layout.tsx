const Layout = (props: Readonly<React.PropsWithChildren>) => {
    return (
        <div className="min-h-screen bg-slate-50">
            {props.children}
        </div>
    );
}

export default Layout;