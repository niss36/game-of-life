import { Inter } from "next/font/google";
import styles from "@/styles/Home.module.css";
import WebGlGameOfLife from "@/components/webGlLife";

const inter = Inter({ subsets: ["latin"] });

export default function WebGl() {
  return (
    <main className={`${styles.main} ${inter.className}`}>
      <WebGlGameOfLife />
    </main>
  );
}
