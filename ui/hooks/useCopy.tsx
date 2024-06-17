import { useToast } from "@/components/ui/use-toast";

const useCopy = () => {
    const { toast } = useToast();
    const copyHandler = (value: string, key: string) => {
        const code = value;
        if (code) {
            navigator.clipboard.writeText(code);
            toast({
                description: `${key} Copied to Clipboard.`,
            });
        }
    };
    return { copyHandler }
}

export default useCopy; 