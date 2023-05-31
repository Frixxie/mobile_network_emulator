<script lang="ts">
    import P5 from "p5-svelte";
    const sketch = (p5: any) => {
        let width = 1000;
        let height = 1000;

        class User {
            x: number;
            y: number;

            constructor(x: number, y: number) {
                this.x = x;
                this.y = y;
            }

            display(c: any) {
                p5.fill(c);
                p5.noStroke();
                p5.circle(this.x + width / 2, this.y + height / 2, 10);
            }
        }

        class Ran {
            x: number;
            y: number;
            radius: number;

            constructor(x: number, y: number, radius: number) {
                this.x = x;
                this.y = y;
                this.radius = radius;
            }

            display() {
                let c = p5.color(0, 255, 0);
                p5.fill(c);
                p5.noStroke();
                p5.circle(
                    this.x + width / 2,
                    this.y + height / 2,
                    this.radius * 2
                );
                c = p5.color(255, 255, 0);
                p5.fill(c);
                p5.noStroke();
                p5.circle(this.x + width / 2, this.y + height / 2, 10);
            }
        }

        class EdgeDataCenter {
            x: number;
            y: number;
            id: number;
            name: string;

            constructor(x: number, y: number, id: number, name: string) {
                this.x = x;
                this.y = y;
                this.id = id;
                this.name = name;
            }

            display() {
                let c = p5.color(255, 0, 0);
                p5.fill(c);
                p5.noStroke();
                p5.rect(this.x + width / 2, this.y + height / 2, 10, 10);
            }
        }

        async function get_users(): Promise<User[]> {
            let res = await fetch("http://localhost:8080/mobile_network/users")
                .then((response) => {
                    return response.json();
                })
                .then((users: any[]) => {
                    let res = [];
                    for (let user of users) {
                        let new_user = new User(user.x, user.y);
                        res.push(new_user);
                    }
                    return res;
                });
            return res;
        }

        async function get_connected_users(): Promise<User[]> {
            let res = await fetch(
                "http://localhost:8080/mobile_network/connected_users"
            )
                .then((response) => {
                    return response.json();
                })
                .then((users: any[]) => {
                    let res = [];
                    for (let user of users) {
                        let new_user = new User(user.user.x, user.user.y);
                        res.push(new_user);
                    }
                    return res;
                });
            return res;
        }

        async function get_rans(): Promise<Ran[]> {
            let res = await fetch("http://localhost:8080/mobile_network/rans")
                .then((response) => {
                    return response.json();
                })
                .then((rans: any[]) => {
                    let res = [];
                    for (let ran of rans) {
                        let new_ran = new Ran(ran.x, ran.y, ran.radius);
                        res.push(new_ran);
                    }
                    return res;
                });
            return res;
        }

        async function get_edcs(): Promise<EdgeDataCenter[]> {
            let res = await fetch(
                "http://localhost:8080/network/edge_data_centers"
            )
                .then((response) => {
                    return response.json();
                })
                .then((edge_data_centers: any[]) => {
                    let res = [];
                    for (let edge_data_center of edge_data_centers) {
                        let new_edge_data_center = new EdgeDataCenter(
                            edge_data_center.x,
                            edge_data_center.y,
                            edge_data_center.id,
                            edge_data_center.name
                        );
                        res.push(new_edge_data_center);
                    }
                    return res;
                });
            return res;
        }

        async function update_user_pos(): Promise<void> {
            let options: RequestInit = {
                method: "POST",
            };
            await fetch(
                "http://localhost:8080/mobile_network/update_user_positions",
                options
            );
            return;
        }

        async function update(): Promise<void> {
            get_users().then((remote_users) => {
                users = remote_users;
            });
            get_connected_users().then((remote_users) => {
                connected_users = remote_users;
            });
            //update_user_pos().then(() => {});
        }

        let user_color = p5.color(100, 100, 200);
        let connected_user_collor = p5.color(0, 0, 200);

        let rans: Ran[] = [];
        let edcs: EdgeDataCenter[] = [];
        let users: User[] = [];
        let connected_users: User[] = [];

        get_rans().then((remote_rans) => {
            rans = remote_rans;
        });
        get_edcs().then((remote_edcs) => {
            edcs = remote_edcs;
        });
        get_users().then((remote_users) => {
            users = remote_users;
        });
        get_connected_users().then((remote_users) => {
            connected_users = remote_users;
        });

        p5.setup = () => {
            p5.createCanvas(width, height);
            p5.background(100, 100, 100);
        };

        p5.draw = () => {
            p5.background(100, 100, 100);
            for (let ran of rans) {
                ran.display();
            }
            for (let edc of edcs) {
                edc.display();
            }
            for (let user of users) {
                user.display(user_color);
            }
            for (let user of connected_users) {
                user.display(connected_user_collor);
            }
            update().then(() => {});
        };
    };
</script>

<P5 {sketch} />
