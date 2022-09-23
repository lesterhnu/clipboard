import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText, readText } from "@tauri-apps/api/clipboard";
import "./App.css";
import {
  appWindow,
  LogicalPosition,
  PhysicalPosition,
} from "@tauri-apps/api/window";
import { sendNotification } from "@tauri-apps/api/notification";
import {
  Grid,
  Paper,
  Box,
  IconButton,
  ListItem,
  List,
  ListItemText,
  Button,
  Collapse,
  Divider,
  Typography,
  Tooltip,
  AppBar,
} from "@mui/material";
import { createTheme, ThemeProvider, styled } from "@mui/material/styles";
import DeleteIcon from "@mui/icons-material/Delete";
import { PushPin, PushPinOutlined } from "@mui/icons-material";
import { deepPurple, teal } from "@mui/material/colors";
import { TransitionGroup } from "react-transition-group";

interface Record {
  id?: number;
  content: string;
  hash_string?: string;
  timestamp: number;
  is_del?: number;
}

const App: React.FC = () => {
  const [isAlwaysOnTop, setIsAlwaysOnTop] = useState(true);
  const [clipboardContent, setClipboardContent] = useState<string>("");
  const [recordList, setRecordList] = useState<Record[]>([]);
  useEffect(() => {
    (async () => {
      let oldcontent = await getClipboardContent();
      let newcontent = oldcontent;
      setInterval(async () => {
        newcontent = await getClipboardContent();
        if (newcontent != oldcontent) {
          setClipboardContent(newcontent);
          oldcontent = newcontent;
        }
      }, 500);
    })();
  }, []);
  useEffect(() => {
    (async () => {
      await insert_if_not_exist();
      let res = await batch_get_record();
      // setRecordList(res)
    })();
  }, [clipboardContent]);
  useEffect(() => {
    (async () => {
      const content = await getClipboardContent();
      setClipboardContent(content);
    })();
  }, [clipboardContent]);
  const batch_get_record = async (): Promise<Record[]> => {
    let res = await invoke<Record[]>("batch_get_record").then((resp) => {
      return resp;
    });
    setRecordList(res);
    return res;
  };
  const set_always_on_top = async () => {
    setIsAlwaysOnTop(!isAlwaysOnTop);
    await appWindow.setAlwaysOnTop(isAlwaysOnTop);
  };
  const set_position = async () => {
    await appWindow.setPosition(new PhysicalPosition(3000, 0));
  };
  const delete_record = async (id: number) => {
    invoke("delete_record", { id })
      .then((resp) => {
        sendNotification({ title: "Deleted" });
        batch_get_record();
      })
      .catch((err) => {
        console.error(err);
        sendNotification({ title: "删除失败", body: err });
      });
  };
  const insert_if_not_exist = async () => {
    let content = await getClipboardContent();
    let d = new Date();
    const r: Record = {
      id: 0,
      content: content,
      timestamp: d.getTime(),
      hash_string: "",
      is_del: 0,
    };
    await invoke("insert_if_not_exist", { r: r });
  };
  const getClipboardContent = async () => {
    return (await readText()) || "";
  };
  const readClipboard = async () => {
    return (await readText()) || "";
  };
  const writeClipboard = async (content: string) => {
    let res = await writeText(content);
  };
  const clear_data = async () => {
    await invoke("clear_data");
    await batch_get_record();
  };
  const writeClipboardContent = async (s: string) => {
    await writeText(s);
    sendNotification({ title: "Copied!", body: s });
  };
  const hideWindow = async () => {
    await appWindow.hide();
  };
  return (
    <div>
      <AppBar color="default">
        <Grid container>
          <Grid item xs={2}>
            <Box>
              <IconButton size="small" onClick={set_always_on_top}>
                {isAlwaysOnTop ? <PushPinOutlined /> : <PushPin />}
              </IconButton>
            </Box>
          </Grid>
          <Grid xs={2}>
            <Box>
              <IconButton
                edge="end"
                size="small"
                aria-label="delete"
                onClick={clear_data}
                title="删除所有"
              >
                <DeleteIcon />
              </IconButton>
            </Box>
          </Grid>
          <Grid xs={4}>
            <Button
          // size="small"
          variant="contained"
          color="secondary"
          onClick={hideWindow}
        >
          隐藏窗口
        </Button>
          </Grid>
        </Grid>

        
      </AppBar>

      <Grid item container sx={{mt:5}}>
        <Grid xs={12}>
          <List>
            <TransitionGroup>
              {recordList.map((item, index) => (
                <Collapse key={index}>
                  <ListItem
                    onClick={() => {
                      writeClipboardContent(item.content);
                    }}
                    sx={{
                      bgcolor: "#ddd",
                      height: 60,
                      mt: 1,
                      borderRadius: "15px",
                      ":hover": {
                        bgcolor: "#ccc",
                        transform: `scale(1.01)`,
                      },
                    }}
                    secondaryAction={
                      <IconButton
                        color="secondary"
                        edge="end"
                        aria-label="delete"
                        onClick={() => {
                          delete_record(item.id as number);
                        }}
                        title="删除本条"
                      >
                        <DeleteIcon color="error" />
                      </IconButton>
                    }
                  >
                    <Tooltip title={item.content} placement="bottom">
                      <ListItemText
                        sx={{
                          overFlow: "hidden",
                          textOverflow: "ellipsis",
                        }}
                        primary={
                          <Typography noWrap={true} sx={{ overflow: "hidden" }}>
                            {item.content}
                          </Typography>
                        }
                        // secondary={item.content}
                      ></ListItemText>
                    </Tooltip>
                  </ListItem>
                </Collapse>
              ))}
            </TransitionGroup>
          </List>
        </Grid>
      </Grid>
    </div>
  );
};

export default App;
