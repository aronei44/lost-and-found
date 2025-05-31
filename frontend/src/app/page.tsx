"use client";
import Layout from "@/components/Layout";
import Navbar from "@/components/Navbar";
import { useGlobalContext } from "@/hooks/globalprovider";

export default function Home() {
  const { state: {
    activeComponent
  }} = useGlobalContext();
  return (
    <Layout>
      <Navbar />
      {activeComponent}
    </Layout>
  );
}
