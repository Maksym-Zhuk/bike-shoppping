import { LinearGradient } from "expo-linear-gradient";
import {type ReactNode} from "react"
export default function Product({children}:{children: ReactNode}){
  return (
    <LinearGradient
    colors={["#34C8E8", "#4E4AF2"]}
    start={{ x: 0, y: 0 }}
    end={{ x: 0, y: 1 }}
    style={{
      flex: 1,
      flexDirection: "row",
      alignItems: "center",
      paddingHorizontal: 10,
    }}
    >
    {children}
    </LinearGradient>
)
}
