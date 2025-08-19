import {Box, Heading, Flex, Image, Card, Text, Group, Input, InputGroup} from '@chakra-ui/react';
import { FiSearch } from 'react-icons/fi';
import axios from 'axios'
import { useEffect, useState, useMemo } from "react";
import logo from '@/assets/logo.png'

interface ApiInfo {
    name: string;
    description: string;
    folder_name: string;
}

export default function Home() {
    const [appName, setAppName] = useState("星柠图片API");
    const getAppName = async () => {
        try {
            const res = await axios.get('/admin/status');
            return res.data.data.app_name;
        } catch {
            return "星柠图片API";
        }
    };
    useEffect(() => {
        getAppName().then((title) => {
            setAppName(title);
            document.title = title;
        });
    }, []);

    const [apiList, setApiList] = useState<ApiInfo[]>([]);
    const [searchTerm, setSearchTerm] = useState('');

    const getApiList = async () => {
        try {
            const res = await axios.get('/admin/infos');
            return res.data.data;
        } catch {
            return [];
        }
    };

    useEffect(() => {
        getApiList().then((list) => {
            if (list) {
                setApiList(list);
            } else {
                setApiList([]);
            }
        });
    }, []);

    const filteredApiList = useMemo(() => {
        if (searchTerm === '') {
            return apiList;
        } else {
            return apiList.filter(api =>
                api.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                api.description.toLowerCase().includes(searchTerm.toLowerCase())
            );
        }
    }, [apiList, searchTerm]);

    const handleSearch = (value: string) => {
        setSearchTerm(value);
    };

    return (
        <Box>
            <Flex
                as="nav"
                backgroundColor={"pink.200"}
            >
                <Flex gap="1rem" className={"flex-1 justify-start items-center"}>
                    <Image
                        src={logo}
                        alt="logo"
                        className={"size-8 md:size-15 !m-1"}
                    />
                    <Heading as={"h2"} className={"text-black text-center"}> {appName}</Heading>

                </Flex>
                {/** TODO: 右侧区域，待完成 */}
                <Flex className={"flex-1 flex-end"} gap="1rem">
                </Flex>
            </Flex>
            {/** 标题 */}
            <Box className={"w-screen h-[200px] md:h-[300px] bg-[url('https://api.wuliya.cn/api/image/loli')] bg-cover bg-center bg-no-repeat"}>
                <Heading as={"h1"} className={"flex justify-center items-center text-black !pt-13"}> {appName}</Heading>
            </Box>
            {/** 搜索按钮 */}
            <Box className="bg-[#fef8fa] h-screen">
                <Box className="flex justify-center items-start !pt-10">
                    <Box className="w-1/2 h-full"
                         borderWidth={"1px"}
                         borderColor={"#c28697"}
                         borderRadius={"10px"}>

                        <InputGroup startElement={<FiSearch />}>
                            <Input
                                className="w-full !bg-transparent"
                                placeholder="搜索"
                                value={searchTerm}
                                onChange={(e) => handleSearch(e.target.value)}
                            />
                        </InputGroup>
                    </Box>
                </Box>

                <Group
                    className={"w-full"}
                    paddingTop={"40px"}
                    gap="20px"
                    wrap="wrap"
                    px="10px"
                    justifyContent="center"
                >
                    {filteredApiList.map((card) => (
                        <ImageCard
                            name={card.name}
                            image_url={`${window.location.origin}/api/${card.folder_name}`}
                            description={card.description}
                        />
                    ))}
                </Group >
            </Box>
        </Box>
    );
}

function ImageCard(props: {
    name: string;
    image_url: string;
    description: string;
}){

    const [imageSrc, setImageSrc] = useState(props.image_url);

    const refreshImage = () => {
        const url = new URL(props.image_url);
        url.searchParams.set('t', Date.now().toString());
        setImageSrc(url.toString());
    };
    return (
        <Card.Root
            className=" w-9/10 md:w-1/5"
        >
            <Image
                onClick={refreshImage}
                src={imageSrc}
                alt={props.name}
            />
            <Card.Body>
                <Text>Tips: 点击图片可刷新预览图</Text>
                <Heading size="md">{props.name}</Heading>
                <Text>{props.description}</Text>
            </Card.Body>
        </Card.Root>
    )
}
